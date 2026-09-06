use std::fs;
use std::path::{Path, PathBuf};

use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::types::{
    Angle, Area, DayInterval, Duration, Gm, ManDc, Mass, NonNegativeDouble, Percentage,
    Probability, Vec3Double,
};
use ccsds_ndm::{Ndm, Validate};

mod common;
use common::{assert_rejects, validate_xml};

const KVN: &str = include_str!("../data/kvn/ocm_g18.kvn");
const XML: &str = include_str!("../data/xml/ocm_g20.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn ocm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let center = "CENTER_NAME = EARTH";
    let frame = "TRAJ_REF_FRAME = TOD_EARTH";
    for (label, source) in [
        (
            "duplicate metadata keyword",
            KVN.replace("TIME_SYSTEM = UTC", "TIME_SYSTEM = UTC\nTIME_SYSTEM = UTC"),
        ),
        (
            "reordered trajectory keywords",
            KVN.replace(&format!("{center}\n{frame}"), &format!("{frame}\n{center}")),
        ),
        (
            "unknown trajectory keyword",
            KVN.replace(center, &format!("{center}\nUNKNOWN = value")),
        ),
        (
            "comment after trajectory content",
            KVN.replace(center, &format!("{center}\nCOMMENT misplaced")),
        ),
        ("unknown block", KVN.replace("TRAJ_START", "UNKNOWN_START")),
        ("mismatched block end", KVN.replace("TRAJ_STOP", "COV_STOP")),
        (
            "out-of-order logical block",
            KVN.replace("PHYS_START", "PERT_START")
                .replace("PHYS_STOP", "PERT_STOP"),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII assignment",
            KVN.replace(center, &format!("{center} €")),
        ),
    ] {
        assert!(Ocm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn ocm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let center = "<CENTER_NAME>EARTH</CENTER_NAME>";
    let time_system = "<TIME_SYSTEM>UT1</TIME_SYSTEM>";
    for (label, source) in [
        (
            "unknown data child",
            XML.replace("<data>", "<data><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown trajectory child",
            XML.replace("<traj>", "<traj><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown leaf attribute",
            XML.replace(center, "<CENTER_NAME unexpected=\"value\">EARTH</CENTER_NAME>"),
        ),
        (
            "duplicate metadata child",
            XML.replace(time_system, &format!("{time_system}{time_system}")),
        ),
        (
            "reordered metadata children",
            XML.replace(
                "<OBJECT_NAME>OSPREY 5</OBJECT_NAME>\n<INTERNATIONAL_DESIGNATOR>2022-999A</INTERNATIONAL_DESIGNATOR>",
                "<INTERNATIONAL_DESIGNATOR>2022-999A</INTERNATIONAL_DESIGNATOR>\n<OBJECT_NAME>OSPREY 5</OBJECT_NAME>",
            ),
        ),
    ] {
        assert!(Ocm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_ocm_fixture_preserves_histories_and_generates_valid_xml() {
    for name in [
        "ocm_g15.kvn",
        "ocm_g16.kvn",
        "ocm_g17.kvn",
        "ocm_g18.kvn",
        "ocm_g19.kvn",
    ] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Ocm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Ocm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Ocm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    let message = Ocm::from_xml(XML).unwrap();
    let xml = message.to_xml().unwrap();
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);
    validate_xml("ocm_g20.xml", &xml);
}

#[test]
fn time_and_angle_vectors_use_the_schema_lexical_form_across_notations() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    let maneuver = &mut message.body.segment.data.man[0];
    maneuver.dc_type = ManDc::TimeAndAngle;
    maneuver.dc_win_open = Some("0".parse().unwrap());
    maneuver.dc_win_close = Some("10".parse().unwrap());
    maneuver.dc_exec_start = Some("1".parse().unwrap());
    maneuver.dc_exec_stop = Some("9".parse().unwrap());
    maneuver.dc_ref_time = Some("0".parse().unwrap());
    maneuver.dc_time_pulse_duration = Some(Duration::new(1.0, None).unwrap());
    maneuver.dc_time_pulse_period = Some(Duration::new(2.0, None).unwrap());
    maneuver.dc_ref_dir = Some(Vec3Double::new(1.0, 0.0, 0.0));
    maneuver.dc_body_frame = Some("SC_BODY".to_owned());
    maneuver.dc_body_trigger = Some(Vec3Double::new(0.0, 1.0, 0.0));
    maneuver.dc_pa_start_angle = Some("0".parse().unwrap());
    maneuver.dc_pa_stop_angle = Some("180".parse().unwrap());

    let xml = message.to_xml().unwrap();
    assert!(xml.contains("<DC_REF_DIR>1 0 0</DC_REF_DIR>"));
    assert!(xml.contains("<DC_BODY_TRIGGER>0 1 0</DC_BODY_TRIGGER>"));
    assert!(!xml.contains("<DC_REF_DIR><x>"));
    validate_xml("TIME_AND_ANGLE vectors", &xml);
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);

    let kvn = message.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&kvn).unwrap(), message);
}

/// `Vec3Double` has three public `f64` components and a constructor that checks nothing, so it
/// is the one fixed-size numeric type in the crate whose values were never validated. An infinity
/// reached the document as `inf 0 0`, which `xmllint` rejects against the `vec3Double` list type.
#[test]
fn time_and_angle_vector_components_must_be_finite() {
    fn time_and_angle_message() -> Ocm {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        let maneuver = &mut message.body.segment.data.man[0];
        maneuver.dc_type = ManDc::TimeAndAngle;
        maneuver.dc_win_open = Some("0".parse().unwrap());
        maneuver.dc_win_close = Some("10".parse().unwrap());
        maneuver.dc_exec_start = Some("1".parse().unwrap());
        maneuver.dc_exec_stop = Some("9".parse().unwrap());
        maneuver.dc_ref_time = Some("0".parse().unwrap());
        maneuver.dc_time_pulse_duration = Some(Duration::new(1.0, None).unwrap());
        maneuver.dc_time_pulse_period = Some(Duration::new(2.0, None).unwrap());
        maneuver.dc_ref_dir = Some(Vec3Double::new(1.0, 0.0, 0.0));
        maneuver.dc_body_frame = Some("SC_BODY".to_owned());
        maneuver.dc_body_trigger = Some(Vec3Double::new(0.0, 1.0, 0.0));
        maneuver.dc_pa_start_angle = Some("0".parse().unwrap());
        maneuver.dc_pa_stop_angle = Some("180".parse().unwrap());
        message
    }

    type Mutation = fn(&mut Ocm, Vec3Double);
    let cases: [(&str, Mutation); 2] = [
        ("DC_REF_DIR", |message, vector| {
            message.body.segment.data.man[0].dc_ref_dir = Some(vector)
        }),
        ("DC_BODY_TRIGGER", |message, vector| {
            message.body.segment.data.man[0].dc_body_trigger = Some(vector)
        }),
    ];

    for (field, mutate) in cases {
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            // Each component in turn, so none of the three is left unreached.
            for vector in [
                Vec3Double::new(bad, 0.0, 0.0),
                Vec3Double::new(0.0, bad, 0.0),
                Vec3Double::new(0.0, 0.0, bad),
            ] {
                let mut message = time_and_angle_message();
                mutate(&mut message, vector);
                assert_rejects(&message, field);
            }
        }
    }

    // The valid vectors still generate schema-valid XML.
    let message = time_and_angle_message();
    message.validate().unwrap();
    validate_xml("TIME_AND_ANGLE finite vectors", &message.to_xml().unwrap());
}

#[test]
fn every_kvn_generation_gate_rejects_invalid_state_before_output() {
    type OcmMutation = fn(&mut Ocm);
    let cases: [(&str, OcmMutation); 2] = [
        ("non-ASCII free text", |message: &mut Ocm| {
            message.body.segment.metadata.object_name = Some("OSPREY €".to_owned());
        }),
        ("overlong keyword record", |message: &mut Ocm| {
            message.body.segment.metadata.object_name = Some("X".repeat(240));
        }),
    ];
    for (label, mutate) in cases {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        mutate(&mut message);
        assert!(message.to_kvn().is_err(), "materialized accepted {label}");
        let mut output = Vec::new();
        assert!(
            message.write_kvn_to(&mut output).is_err(),
            "streaming accepted {label}"
        );
        assert!(output.is_empty(), "streaming wrote bytes for {label}");
    }
}

#[test]
fn kvn_generation_rounds_trajectory_numbers_to_the_ccsds_digit_limit() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.traj[0].traj_lines[0].values[0] = 1.234_567_890_123_456_7;
    assert!(message.to_kvn().unwrap().contains("1.234567890123457e0"));
}

#[test]
fn edited_ocm_block_values_are_revalidated_before_output() {
    type PhysMutation = fn(&mut ccsds_ndm::messages::ocm::OcmPhysicalDescription);
    let physical: [(&str, PhysMutation); 10] = [
        ("DRAG_COEFF_NOM", |phys| {
            phys.drag_coeff_nom = Some(f64::NAN)
        }),
        ("DRAG_CONST_AREA", |phys| {
            phys.drag_const_area = Some(Area {
                value: -1.0,
                units: None,
            })
        }),
        ("RCS_MAX", |phys| {
            phys.rcs_max = Some(Area {
                value: f64::INFINITY,
                units: None,
            })
        }),
        ("WET_MASS", |phys| {
            phys.wet_mass = Some(Mass {
                value: -0.5,
                units: None,
            })
        }),
        ("DRAG_UNCERTAINTY", |phys| {
            phys.drag_uncertainty = Some(Percentage {
                value: 100.5,
                units: None,
            })
        }),
        ("REFLECTANCE", |phys| {
            phys.reflectance = Some(Probability { value: 1.5 })
        }),
        ("ATT_POINTING", |phys| {
            phys.att_pointing = Some(Angle {
                value: 360.0,
                units: None,
            })
        }),
        ("SOLAR_RAD_COEFF", |phys| {
            phys.solar_rad_coeff = Some(f64::NEG_INFINITY)
        }),
        ("OEB_Q1", |phys| phys.oeb_q1 = Some(f64::NAN)),
        ("IXX", |phys| {
            phys.ixx = Some(ccsds_ndm::types::Moment {
                value: f64::NAN,
                units: None,
            })
        }),
    ];
    for (field, mutate) in physical {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        mutate(message.body.segment.data.phys.as_mut().unwrap());
        assert_rejects(&message, field);
    }

    type PertMutation = fn(&mut ccsds_ndm::messages::ocm::OcmPerturbations);
    let perturbations: [(&str, PertMutation); 3] = [
        ("GM", |pert| {
            pert.gm = Some(Gm {
                value: f64::NAN,
                units: None,
            })
        }),
        ("OBLATE_FLATTENING", |pert| {
            pert.oblate_flattening = Some(f64::NAN)
        }),
        ("FIXED_F10P7", |pert| {
            pert.fixed_f10p7 = Some(ccsds_ndm::types::SolarFlux {
                value: f64::INFINITY,
                units: None,
            })
        }),
    ];
    for (field, mutate) in perturbations {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        mutate(message.body.segment.data.pert.as_mut().unwrap());
        assert_rejects(&message, field);
    }

    type OdMutation = fn(&mut ccsds_ndm::messages::ocm::OcmOdParameters);
    let od: [(&str, OdMutation); 5] = [
        ("WEIGHTED_RMS", |od| {
            od.weighted_rms = Some(NonNegativeDouble { value: f64::NAN })
        }),
        ("GDOP", |od| od.gdop = Some(f64::INFINITY)),
        ("ACTUAL_OD_SPAN", |od| {
            od.actual_od_span = Some(DayInterval {
                value: -1.0,
                units: None,
            })
        }),
        ("OD_CONFIDENCE", |od| {
            od.od_confidence = Some(Percentage {
                value: 101.0,
                units: None,
            })
        }),
        ("OD_EPOCH_EIGMAJ", |od| {
            od.od_epoch_eigmaj = Some(ccsds_ndm::types::Length {
                value: f64::NAN,
                units: None,
            })
        }),
    ];
    for (field, mutate) in od {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        mutate(message.body.segment.data.od.as_mut().unwrap());
        assert_rejects(&message, field);
    }

    // History lines carry raw numbers; the repeated container must be revisited beyond index 0.
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.traj[0].traj_lines[1].values[2] = f64::NAN;
    assert_rejects(&message, "trajLine 2 value 3");
}

#[test]
fn ocm_block_value_boundaries_generate_valid_xml() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    let data = &mut message.body.segment.data;

    let phys = data.phys.as_mut().unwrap();
    phys.drag_coeff_nom = Some(f64::MIN_POSITIVE);
    phys.drag_const_area = Some(Area {
        value: 0.0,
        units: None,
    });
    phys.wet_mass = Some(Mass {
        value: 0.0,
        units: None,
    });
    phys.drag_uncertainty = Some(Percentage {
        value: 100.0,
        units: None,
    });
    phys.reflectance = Some(Probability { value: 1.0 });
    phys.att_pointing = Some(Angle {
        value: -360.0,
        units: None,
    });

    let od = data.od.as_mut().unwrap();
    od.weighted_rms = Some(NonNegativeDouble { value: 0.0 });
    od.gdop = Some(0.0);
    od.od_confidence = Some(Percentage {
        value: 0.0,
        units: None,
    });
    od.actual_od_span = Some(DayInterval {
        value: 0.0,
        units: None,
    });

    message.validate().unwrap();
    let xml = message.to_xml().unwrap();
    validate_xml("OCM block value boundaries", &xml);
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);
}

/// `ManLine::values` is `Vec<String>` because maneuver columns are heterogeneous, and the XSD
/// types the line as a string list, so the schema oracle accepts text in a numeric column. Every
/// column that MAN_COMPOSITION declares numeric must therefore be checked directly. Matches
/// Orekit's `ManeuverFieldType`, which parses declared-numeric columns as doubles and likewise
/// imposes no per-column domain.
#[test]
fn maneuver_line_numeric_columns_must_hold_numbers() {
    for bad in ["abc", "", "NaN", "inf", "1.0e"] {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        // Column 1 of this fixture's composition after the time tag is THR_X.
        message.body.segment.data.man[0].man_lines[0].values[1] = bad.to_owned();
        assert_rejects(&message, "THR_X");
    }

    // A non-numeric column keeps accepting its flag value.
    let mut message = Ocm::from_kvn(KVN).unwrap();
    let interp = message.body.segment.data.man[0].man_lines[0]
        .values
        .iter()
        .position(|value| value == "ON")
        .expect("fixture has a THR_INTERP column");
    message.body.segment.data.man[0].man_lines[0].values[interp] = "OFF".to_owned();
    message
        .validate()
        .expect("THR_INTERP is a flag, not a number");

    // A later record is reached too.
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.man[0].man_lines[1].values[1] = "abc".to_owned();
    assert_rejects(&message, "THR_X");
}

/// Three OCM values have a book domain wider than the 3.0 schema's, so they follow the same
/// P3/P4 split as RDM's `NOMINAL_IMPACT_ALT`: the model preserves the book-valid value, `validate`
/// enforces only finiteness or sign-free finiteness, and XML generation refuses the conversion.
#[test]
fn ocm_book_wider_than_xsd_values_are_refused_at_xml_only() {
    // ODM permits any finite phase angle; `angleType` is [-360, 360).
    for outside in [360.0, -400.0] {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        let maneuver = &mut message.body.segment.data.man[0];
        maneuver.dc_type = ManDc::TimeAndAngle;
        maneuver.dc_win_open = Some("0".parse().unwrap());
        maneuver.dc_win_close = Some("10".parse().unwrap());
        maneuver.dc_exec_start = Some("1".parse().unwrap());
        maneuver.dc_exec_stop = Some("9".parse().unwrap());
        maneuver.dc_ref_time = Some("0".parse().unwrap());
        maneuver.dc_time_pulse_duration = Some(Duration::new(1.0, None).unwrap());
        maneuver.dc_time_pulse_period = Some(Duration::new(2.0, None).unwrap());
        maneuver.dc_ref_dir = Some(Vec3Double::new(1.0, 0.0, 0.0));
        maneuver.dc_body_frame = Some("SC_BODY".to_owned());
        maneuver.dc_body_trigger = Some(Vec3Double::new(0.0, 1.0, 0.0));
        maneuver.dc_pa_start_angle = Some(Angle {
            value: outside,
            units: None,
        });
        maneuver.dc_pa_stop_angle = Some(Angle {
            value: 180.0,
            units: None,
        });

        message
            .validate()
            .expect("ODM imposes no phase-angle range");
        assert!(message.to_kvn().is_ok(), "KVN can represent {outside}");

        let error = message.to_xml().expect_err("angleType cannot represent it");
        assert!(error.to_string().contains("DC_PA_START_ANGLE"), "{error}");
        let mut output = Vec::new();
        assert!(message.write_xml_to(&mut output).is_err());
        assert!(output.is_empty(), "streaming wrote bytes for {outside}");
    }

    // ODM permits signed days-since-observation; the XSD uses a non-negative day interval.
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message
        .body
        .segment
        .data
        .od
        .as_mut()
        .expect("fixture has an OD block")
        .days_since_first_obs = Some(DayInterval {
        value: -3.0,
        units: None,
    });
    message.validate().expect("ODM permits signed values");
    assert!(message.to_kvn().is_ok());
    let error = message
        .to_xml()
        .expect_err("dayIntervalTypeUO is non-negative");
    assert!(
        error.to_string().contains("DAYS_SINCE_FIRST_OBS"),
        "{error}"
    );

    // Non-finite remains a semantic failure in both notations.
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message
        .body
        .segment
        .data
        .od
        .as_mut()
        .unwrap()
        .days_since_last_obs = Some(DayInterval {
        value: f64::NAN,
        units: None,
    });
    assert_rejects(&message, "DAYS_SINCE_LAST_OBS");

    // The accepted boundary still reaches the reference schema.
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message
        .body
        .segment
        .data
        .od
        .as_mut()
        .unwrap()
        .days_since_first_obs = Some(DayInterval {
        value: 0.0,
        units: None,
    });
    message.validate().unwrap();
    validate_xml("OCM days-since boundary", &message.to_xml().unwrap());
}
