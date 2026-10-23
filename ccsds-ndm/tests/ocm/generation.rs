use crate::common::{assert_rejects, mutated, validate_xml};
use crate::KVN;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::types::{
    Angle, Area, DayInterval, Duration, Gm, ManDc, Mass, NonNegativeDouble, Percentage,
    Probability, RevNumBasis, Vec3Double,
};
use ccsds_ndm::{Ndm, Validate};
/// A fixture maneuver promoted to `TIME_AND_ANGLE`, whose required companion fields are all
/// populated. Three tests need this same starting point and then diverge on one value.
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

#[test]
fn time_and_angle_vectors_use_the_schema_lexical_form_across_notations() {
    let message = time_and_angle_message();

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
            for (component, vector) in [
                ("x", Vec3Double::new(bad, 0.0, 0.0)),
                ("y", Vec3Double::new(0.0, bad, 0.0)),
                ("z", Vec3Double::new(0.0, 0.0, bad)),
            ] {
                let mut message = time_and_angle_message();
                mutate(&mut message, vector);
                assert_rejects(&message, &format!("{field} {component}"));
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
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.metadata.object_name = Some("OSPREY €".to_owned());
    crate::common::assert_validation_field(&message.to_kvn().unwrap_err(), "printable ASCII");
    let mut output = Vec::new();
    crate::common::assert_validation_field(
        &message.write_kvn_to(&mut output).unwrap_err(),
        "printable ASCII",
    );
    assert!(output.is_empty());
}

#[test]
fn ocm_accepts_arbitrary_line_lengths() {
    let long_name = "X".repeat(300);
    let source = mutated(
        KVN,
        "OBJECT_NAME = OSPREY 5",
        &format!("OBJECT_NAME = {long_name}"),
    );
    let message = Ocm::from_kvn(&source).unwrap();
    let generated = message.to_kvn().unwrap();
    assert!(generated.lines().any(|line| line.len() > 254));
    let mut streamed = Vec::new();
    message.write_kvn_to(&mut streamed).unwrap();
    assert_eq!(streamed, generated.as_bytes());
}

#[test]
fn kvn_generation_rounds_trajectory_numbers_to_the_ccsds_digit_limit() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.traj[0].traj_lines[0].values[0] = 1.234_567_890_123_456_7;
    assert!(message.to_kvn().unwrap().contains("1.234567890123457"));
}

#[test]
fn kvn_scalar_measures_wrappers_and_vectors_use_the_same_digit_limit() {
    let mut message = time_and_angle_message();
    let value = 0.123_456_789_012_345_67;
    let physical = message.body.segment.data.phys.as_mut().unwrap();
    physical.drag_coeff_nom = Some(value);
    physical.drag_const_area.as_mut().unwrap().value = value;
    physical.reflectance = Some(Probability::new(value).unwrap());
    message
        .body
        .segment
        .data
        .pert
        .as_mut()
        .unwrap()
        .oblate_flattening = Some(value);
    let od = message.body.segment.data.od.as_mut().unwrap();
    od.gdop = Some(value);
    od.weighted_rms = Some(NonNegativeDouble::new(value).unwrap());
    message.body.segment.data.man[0]
        .dc_ref_dir
        .as_mut()
        .unwrap()
        .x = value;
    let kvn = message.to_kvn().unwrap();
    for key in [
        "DRAG_COEFF_NOM",
        "DRAG_CONST_AREA",
        "REFLECTANCE",
        "OBLATE_FLATTENING",
        "GDOP",
        "WEIGHTED_RMS",
        "DC_REF_DIR",
    ] {
        let line = kvn.lines().find(|line| line.starts_with(key)).unwrap();
        assert!(
            line.split_once('=')
                .unwrap()
                .1
                .trim_start()
                .starts_with("1.234567890123457e-1"),
            "{line}"
        );
    }
    Ocm::from_kvn(&kvn).unwrap();
    assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);

    message
        .body
        .segment
        .data
        .phys
        .as_mut()
        .unwrap()
        .drag_const_area
        .as_mut()
        .unwrap()
        .value = f64::MAX;
    message.validate().unwrap();
    assert!(message.to_kvn().is_err());
    let mut output = Vec::new();
    assert!(message.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty());
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
    assert_rejects(&message, "TRAJ data line 2 value 3");
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
/// types `manLine` as `xsd:string`, so the schema oracle accepts text in a numeric column. Every
/// column that MAN_COMPOSITION declares numeric must therefore be checked directly. Matches
/// Orekit's `ManeuverFieldType`, which parses declared-numeric columns as doubles and likewise
/// imposes no per-column domain.
#[test]
fn maneuver_line_numeric_columns_must_hold_numbers() {
    for bad in [
        "abc", "", "NaN", "inf", "1.0e", "1e400", "1_000", "0x1p0", "1junk", " 1", "1 ",
    ] {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        // Column 1 of this fixture's composition after the time tag is THR_X.
        message.body.segment.data.man[0].man_lines[0].values[1] = bad.to_owned();
        assert_rejects(&message, "MAN data line THR_X");
    }

    for value in [
        "-0.0",
        "+1.25",
        "1e-400",
        "5e-324",
        "1.7976931348623157e308",
    ] {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        message.body.segment.data.man[0].man_lines[0].values[1] = value.into();
        message.validate().unwrap();
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
    assert_rejects(&message, "MAN data line THR_X");
}

#[test]
fn maneuver_rows_require_exactly_one_token_per_declared_column() {
    let message = Ocm::from_kvn(KVN).unwrap();
    for extra in [false, true] {
        let mut edited = message.clone();
        let values = &mut edited.body.segment.data.man[0].man_lines[0].values;
        if extra {
            values.push("undeclared".into());
        } else {
            values.pop();
        }
        assert_rejects(&edited, "MAN data line");
    }

    // THR_INTERP is text, but it still occupies exactly one whitespace-delimited column.
    let column = message.body.segment.data.man[0].man_lines[0]
        .values
        .iter()
        .position(|value| value == "ON")
        .unwrap();
    for invalid in ["", "ON OFF", "ON\tOFF", "ON\nOFF"] {
        let mut edited = message.clone();
        edited.body.segment.data.man[0].man_lines[0].values[column] = invalid.into();
        assert_rejects(&edited, "MAN data line THR_INTERP");
    }
}

#[test]
fn maneuver_numeric_precision_keeps_text_identifiers_and_legal_spellings() {
    for composition in [
        "TIME_RELATIVE, DEPLOY_ID, DEPLOY_MASS",
        "TIME_RELATIVE DEPLOY_ID DEPLOY_MASS",
    ] {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        let maneuver = &mut message.body.segment.data.man[0];
        maneuver.man_composition = composition.into();
        maneuver.man_units = None;
        maneuver.man_lines.truncate(1);
        maneuver.man_lines[0].epoch = "0".parse().unwrap();
        maneuver.man_lines[0].values =
            vec!["12345678901234567".into(), "-0.30000000000000004".into()];
        assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
        let kvn = message.to_kvn().unwrap();
        let parsed = Ocm::from_kvn(&kvn).unwrap();
        assert_eq!(
            parsed.body.segment.data.man[0].man_lines[0].values,
            ["12345678901234567", "-0.3"]
        );
        message.body.segment.data.man[0].man_lines[0].values[1] = "-0.2500".into();
        let parsed = Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap();
        assert_eq!(
            parsed.body.segment.data.man[0].man_lines[0].values[1],
            "-0.2500"
        );
        for zero in ["-0", "-0.0", "-0.00e+12"] {
            message.body.segment.data.man[0].man_lines[0].values = vec![zero.into(), zero.into()];
            assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
            let parsed = Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap();
            assert_eq!(
                parsed.body.segment.data.man[0].man_lines[0].values,
                [zero, "0.0"]
            );
        }
        for invalid in [f64::MAX.to_string(), "1e-999".into()] {
            message.body.segment.data.man[0].man_lines[0].values[1] = invalid;
            assert!(message.to_kvn().is_err());
            let mut output = Vec::new();
            assert!(message.write_kvn_to(&mut output).is_err());
            assert!(output.is_empty());
        }
    }
}

/// Some OCM values have a book domain wider than the 3.0 schema's, so they follow the same
/// P3/P4 split as RDM's `NOMINAL_IMPACT_ALT`: the model preserves the book-valid value, `validate`
/// enforces only finiteness or sign-free finiteness, and XML generation refuses the conversion.
#[test]
fn ocm_book_wider_than_xsd_values_are_refused_at_xml_only() {
    // ODM says zero disables atmospheric drag; the XSD requires a positive value.
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message
        .body
        .segment
        .data
        .phys
        .as_mut()
        .unwrap()
        .drag_coeff_nom = Some(0.0);
    message.validate().expect("ODM permits zero");
    assert!(message.to_kvn().is_ok());
    let error = message.to_xml().expect_err("positiveDouble excludes zero");
    assert!(error.to_string().contains("DRAG_COEFF_NOM"), "{error}");

    // ODM permits any finite phase angle; `angleType` is [-360, 360).
    for outside in [360.0, -400.0] {
        let mut message = time_and_angle_message();
        message.body.segment.data.man[0].dc_pa_start_angle = Some(Angle {
            value: outside,
            units: None,
        });

        message
            .validate()
            .expect("ODM imposes no phase-angle range");
        assert!(message.to_kvn().is_ok(), "KVN can represent {outside}");

        let error = message.to_xml().expect_err("angleType cannot represent it");
        assert!(error.to_string().contains("DC_PA_START_ANGLE"), "{error}");
        let mut output = Vec::new();
        crate::common::assert_validation_field(
            &message.write_xml_to(&mut output).unwrap_err(),
            "DC_PA_START_ANGLE",
        );
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

#[test]
fn ocm_reports_the_unrepresentable_value_path_compactly() {
    let mut message = Ocm::from_kvn(include_str!("../../data/kvn/ocm_g15.kvn")).unwrap();
    message.body.segment.data.traj[0].traj_lines[0].values[0] = f64::MAX;

    let error = message.to_kvn().unwrap_err();
    assert_eq!(
        error
            .as_validation_error()
            .and_then(|error| error.field_path()),
        Some("body.segment.data.traj[0].traj_lines[0].values[0]".into())
    );
    assert!(error.to_string().contains("1.7976931348623157e308"));
}

#[test]
fn kvn_roundtrip() {
    // Full roundtrip: KVN -> Ocm -> KVN
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1000 2000 3000 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let output = ocm.to_kvn().unwrap();

    // Parse output again
    let ocm2 = Ocm::from_kvn(&output).unwrap();
    assert_eq!(ocm2, ocm);
}

#[test]
fn od_optional_fields_roundtrip() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
PERT_STOP
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 30 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
RECOMMENDED_OD_SPAN = 7 [d]
ACTUAL_OD_SPAN = 7.5 [d]
OBS_AVAILABLE = 1000
OBS_USED = 950
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
MAXIMUM_OBS_GAP = 0.5 [d]
OD_EPOCH_EIGMAJ = 100 [m]
OD_EPOCH_EIGINT = 50 [m]
OD_EPOCH_EIGMIN = 25 [m]
OD_MAX_PRED_EIGMAJ = 200 [m]
OD_MIN_PRED_EIGMIN = 10 [m]
OD_CONFIDENCE = 95 [%]
GDOP = 1.5
SOLVE_N = 6
SOLVE_STATES = X Y Z VX VY VZ
CONSIDER_N = 2
CONSIDER_PARAMS = CD CR
SEDR = 0.001 [W/kg]
SENSORS_N = 3
SENSORS = SENSOR_A SENSOR_B SENSOR_C
WEIGHTED_RMS = 1.2
DATA_TYPES = RANGE DOPPLER
OD_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let od = ocm.body.segment.data.od.as_ref().unwrap();

    // Verify all fields were parsed
    assert_eq!(od.od_prev_id, Some("OD0".to_string()));
    assert!(od.actual_od_span.is_some());
    assert_eq!(od.obs_available, Some(1000));
    assert_eq!(od.obs_used, Some(950));
    assert_eq!(od.tracks_available, Some(50));
    assert_eq!(od.tracks_used, Some(48));
    assert!(od.maximum_obs_gap.is_some());
    assert!(od.od_epoch_eigmaj.is_some());
    assert!(od.od_epoch_eigint.is_some());
    assert!(od.od_epoch_eigmin.is_some());
    assert!(od.od_max_pred_eigmaj.is_some());
    assert!(od.od_min_pred_eigmin.is_some());
    assert!(od.od_confidence.is_some());
    assert_eq!(od.gdop, Some(1.5));
    assert_eq!(od.solve_n, Some(6));
    assert!(od.solve_states.is_some());
    assert_eq!(od.consider_n, Some(2));
    assert!(od.consider_params.is_some());
    assert!(od.sedr.is_some());
    assert_eq!(od.sensors_n, Some(3));
    assert_eq!(od.sensors, Some("SENSOR_A SENSOR_B SENSOR_C".to_string()));
    assert_eq!(od.weighted_rms, Some(NonNegativeDouble::new(1.2).unwrap()));
    assert_eq!(od.data_types, Some("RANGE DOPPLER".to_string()));

    let output = ocm.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), ocm);
}

#[test]
fn test_ocm_data_write_kvn_all_blocks() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 0.000001 0 0.000001 0 0 0.000001 0 0 0 0.000001 0 0 0 0 0.000001 0 0 0 0 0 0.000001
COV_STOP
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
PERT_STOP
OD_START
OD_ID = OD1
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
USER_START
COMMENT user comment
USER_DEFINED_CUSTOM_PARAM = custom_value
USER_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let output = ocm.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), ocm);
}

#[test]
fn test_cov_write_kvn_all_optional_fields() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COMMENT cov comment
COV_ID = COV_001
COV_PREV_ID = COV_000
COV_NEXT_ID = COV_002
COV_BASIS = DETERMINED
COV_BASIS_ID = BASIS_001
COV_REF_FRAME = RSW
COV_FRAME_EPOCH = 2023-01-01T00:00:00
COV_SCALE_MIN = 0.5
COV_SCALE_MAX = 2.0
COV_CONFIDENCE = 95 [%]
COV_TYPE = CARTPV
COV_ORDERING = LTM
COV_UNITS = [km, km, km, km/s, km/s, km/s]
2023-01-01T00:00:00 0.000001 0 0.000001 0 0 0.000001 0 0 0 0.000001 0 0 0 0 0.000001 0 0 0 0 0 0.000001
COV_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let cov = &ocm.body.segment.data.cov[0];

    // Verify all optional fields were parsed
    assert_eq!(cov.comment, vec!["cov comment"]);
    assert_eq!(cov.cov_id, Some("COV_001".to_string()));
    assert_eq!(cov.cov_prev_id, Some("COV_000".to_string()));
    assert_eq!(cov.cov_next_id, Some("COV_002".to_string()));
    assert!(cov.cov_basis.is_some());
    assert_eq!(cov.cov_basis_id, Some("BASIS_001".to_string()));
    assert!(cov.cov_frame_epoch.is_some());
    assert_eq!(cov.cov_scale_min, Some(0.5));
    assert_eq!(cov.cov_scale_max, Some(2.0));
    assert!(cov.cov_confidence.is_some());
    assert_eq!(
        cov.cov_units,
        Some("[km, km, km, km/s, km/s, km/s]".to_string())
    );

    let output = ocm.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), ocm);
}

#[test]
fn test_man_all_optional_fields_write_kvn() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MAN_1
MAN_PREV_ID = MAN_0
MAN_NEXT_ID = MAN_2
MAN_BASIS = PLANNED
MAN_BASIS_ID = PLAN_001
MAN_DEVICE_ID = THRUSTER_1
MAN_PREV_EPOCH = 2022-12-31T00:00:00
MAN_NEXT_EPOCH = 2023-01-02T00:00:00
MAN_PURPOSE = ORBIT_RAISING
MAN_PRED_SOURCE = FDSS
MAN_REF_FRAME = RSW
MAN_FRAME_EPOCH = 2023-01-01T00:00:00
GRAV_ASSIST_NAME = MOON
DC_TYPE = TIME
DC_WIN_OPEN = 2023-01-01T00:00:00
DC_WIN_CLOSE = 2023-01-01T02:00:00
DC_MIN_CYCLES = 1
DC_MAX_CYCLES = 10
DC_EXEC_START = 2023-01-01T00:30:00
DC_EXEC_STOP = 2023-01-01T01:30:00
DC_REF_TIME = 2023-01-01T01:00:00
DC_TIME_PULSE_DURATION = 60 [s]
DC_TIME_PULSE_PERIOD = 120 [s]
DC_REF_DIR = 1 0 0
DC_BODY_FRAME = SC_BODY
DC_BODY_TRIGGER = 0 1 0
DC_PA_START_ANGLE = 0 [deg]
DC_PA_STOP_ANGLE = 180 [deg]
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
MAN_UNITS = km/s km/s km/s
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let man = &ocm.body.segment.data.man[0];

    // Verify optional fields
    assert_eq!(man.man_prev_id, Some("MAN_0".to_string()));
    assert_eq!(man.man_next_id, Some("MAN_2".to_string()));
    assert!(man.man_basis.is_some());
    assert!(man.grav_assist_name.is_some());
    assert!(man.dc_win_open.is_some());
    assert!(man.dc_min_cycles.is_some());
    assert!(man.dc_ref_dir.is_some());
    assert!(man.dc_body_trigger.is_some());
    assert!(man.dc_pa_start_angle.is_some());

    let output = ocm.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), ocm);
}

#[test]
fn test_pert_all_optional_fields_write_kvn() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008 70x70
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = MOON SUN JUPITER
CENTRAL_BODY_ROTATION = 7.2921e-5 [deg/s]
OBLATE_FLATTENING = 0.003353
OCEAN_TIDES_MODEL = GOT4.7
SOLID_TIDES_MODEL = IERS2010
REDUCTION_THEORY = IERS2010
ALBEDO_MODEL = EARTH_ALBEDO
ALBEDO_GRID_SIZE = 36
SHADOW_MODEL = CYLINDRICAL
SHADOW_BODIES = MOON
SRP_MODEL = FLAT_PLATE
SW_DATA_SOURCE = CSSI
SW_DATA_EPOCH = 2023-01-01T00:00:00
SW_INTERP_METHOD = LINEAR
FIXED_GEOMAG_KP = 3 [nT]
FIXED_GEOMAG_AP = 15 [nT]
FIXED_GEOMAG_DST = -10 [nT]
FIXED_F10P7 = 150 [SFU]
FIXED_F10P7_MEAN = 145 [SFU]
FIXED_M10P7 = 148 [SFU]
FIXED_M10P7_MEAN = 143 [SFU]
FIXED_S10P7 = 147 [SFU]
FIXED_S10P7_MEAN = 142 [SFU]
FIXED_Y10P7 = 146 [SFU]
FIXED_Y10P7_MEAN = 141 [SFU]
PERT_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let pert = ocm.body.segment.data.pert.as_ref().unwrap();

    // Verify all fields
    assert!(pert.central_body_rotation.is_some());
    assert!(pert.oblate_flattening.is_some());
    assert!(pert.albedo_grid_size.is_some());
    assert!(pert.sw_data_epoch.is_some());
    assert!(pert.fixed_geomag_kp.is_some());
    assert!(pert.fixed_m10p7.is_some());

    let output = ocm.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), ocm);
}

#[test]
fn test_ocm_metadata_all_fields() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
COMMENT meta comment
OBJECT_NAME = SAT1
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 12345
ALTERNATE_NAMES = SAT_ALT
ORIGINATOR_POC = JOHN DOE
ORIGINATOR_POSITION = ENGINEER
ORIGINATOR_PHONE = 123-456
ORIGINATOR_EMAIL = john@example.com
ORIGINATOR_ADDRESS = 123 Street
TECH_ORG = SPACE_CORP
TECH_POC = JANE DOE
TECH_POSITION = SCIENTIST
TECH_PHONE = 987-654
TECH_EMAIL = jane@example.com
TECH_ADDRESS = 456 Avenue
PREVIOUS_MESSAGE_ID = MSG_001
NEXT_MESSAGE_ID = MSG_003
ADM_MSG_LINK = ADM_LINK
CDM_MSG_LINK = CDM_LINK
PRM_MSG_LINK = PRM_LINK
RDM_MSG_LINK = RDM_LINK
TDM_MSG_LINK = TDM_LINK
OPERATOR = OPS_TEAM
OWNER = OWNER_TEAM
COUNTRY = USA
CONSTELLATION = STARLINK
OBJECT_TYPE = PAYLOAD
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
OPS_STATUS = OPERATIONAL
ORBIT_CATEGORY = LEO
OCM_DATA_ELEMENTS = ALL
SCLK_OFFSET_AT_EPOCH = 0.1 [s]
SCLK_SEC_PER_SI_SEC = 0.99 [s]
PREVIOUS_MESSAGE_EPOCH = 2022-12-31T23:00:00
NEXT_MESSAGE_EPOCH = 2023-01-01T01:00:00
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
TIME_SPAN = 1.0 [d]
TAIMUTC_AT_TZERO = 37.0 [s]
NEXT_LEAP_EPOCH = 2024-01-01T00:00:00
NEXT_LEAP_TAIMUTC = 38.0 [s]
UT1MUTC_AT_TZERO = -0.1 [s]
EOP_SOURCE = IERS
INTERP_METHOD_EOP = LINEAR
CELESTIAL_SOURCE = IAU
META_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let meta = &ocm.body.segment.metadata;
    assert_eq!(meta.object_name, Some("SAT1".to_string()));
    assert_eq!(meta.object_type.as_deref(), Some("PAYLOAD"));
    assert!(meta.sclk_offset_at_epoch.is_some());

    let output = ocm.to_kvn().unwrap();
    let ocm2 = Ocm::from_kvn(&output).unwrap();
    assert_eq!(ocm2, ocm);
}

#[test]
fn test_traj_all_fields() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
COMMENT traj comment
TRAJ_ID = T1
TRAJ_PREV_ID = T0
TRAJ_NEXT_ID = T2
TRAJ_BASIS = PREDICTED
TRAJ_BASIS_ID = B1
INTERPOLATION = LINEAR
INTERPOLATION_DEGREE = 1
PROPAGATOR = SGP4
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_FRAME_EPOCH = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T00:00:00
USEABLE_STOP_TIME = 2023-01-02T00:00:00
ORB_REVNUM = 1234
ORB_REVNUM_BASIS = 1
TRAJ_TYPE = CARTPV
ORB_AVERAGING = NONE
TRAJ_UNITS = km km km km/s km/s km/s
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let traj = &ocm.body.segment.data.traj[0];
    assert_eq!(traj.traj_id, Some("T1".to_string()));
    assert_eq!(traj.traj_basis.as_deref(), Some("PREDICTED"));
    assert_eq!(traj.orb_revnum_basis, Some(RevNumBasis::One));

    let output = ocm.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), ocm);
}

#[test]
fn test_phys_all_fields_robust() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME
BUS_MODEL = B1
DOCKED_WITH = SAT2
DRAG_CONST_AREA = 1.0 [m**2]
DRAG_COEFF_NOM = 2.2
DRAG_UNCERTAINTY = 10 [%]
INITIAL_WET_MASS = 1000 [kg]
WET_MASS = 900 [kg]
DRY_MASS = 800 [kg]
OEB_PARENT_FRAME = GCRF
OEB_PARENT_FRAME_EPOCH = 2023-01-01T00:00:00
OEB_Q1 = 0
OEB_Q2 = 0
OEB_Q3 = 0
OEB_QC = 1
OEB_MAX = 5 [m]
OEB_INT = 3 [m]
OEB_MIN = 2 [m]
AREA_ALONG_OEB_MAX = 15 [m**2]
AREA_ALONG_OEB_INT = 10 [m**2]
AREA_ALONG_OEB_MIN = 6 [m**2]
AREA_MIN_FOR_PC = 5 [m**2]
AREA_MAX_FOR_PC = 20 [m**2]
AREA_TYP_FOR_PC = 10 [m**2]
RCS = 1 [m**2]
RCS_MIN = 0.5 [m**2]
RCS_MAX = 2 [m**2]
SRP_CONST_AREA = 12 [m**2]
SOLAR_RAD_COEFF = 1.5
SOLAR_RAD_UNCERTAINTY = 5 [%]
VM_ABSOLUTE = 4.5
VM_APPARENT_MIN = 5.0
VM_APPARENT = 5.5
VM_APPARENT_MAX = 6.0
REFLECTANCE = 0.8
ATT_CONTROL_MODE = THREE_AXIS
ATT_ACTUATOR_TYPE = REACTION_WHEELS
ATT_KNOWLEDGE = 0.1 [deg]
ATT_CONTROL = 0.5 [deg]
ATT_POINTING = 0.2 [deg]
AVG_MANEUVER_FREQ = 12 [#/yr]
MAX_THRUST = 0.1 [N]
DV_BOL = 0.5 [km/s]
DV_REMAINING = 0.2 [km/s]
IXX = 100 [kg*m**2]
IYY = 150 [kg*m**2]
IZZ = 150 [kg*m**2]
IXY = 1 [kg*m**2]
IXZ = 2 [kg*m**2]
IYZ = 3 [kg*m**2]
PHYS_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let phys = ocm.body.segment.data.phys.as_ref().unwrap();
    assert_eq!(phys.manufacturer, Some("ACME".to_string()));
    assert_eq!(phys.ixx.as_ref().unwrap().value, 100.0);

    let output = ocm.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), ocm);
}

#[test]
fn test_traj_orb_revnum_basis_zero() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
ORB_REVNUM = 100
ORB_REVNUM_BASIS = 0
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(
        ocm.body.segment.data.traj[0].orb_revnum_basis,
        Some(RevNumBasis::Zero)
    );

    let output = ocm.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), ocm);
}

#[test]
fn xml_only_facets_are_checked_on_read_and_generation() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.traj.truncate(1);
    message.body.segment.data.traj[0].interpolation_degree = Some(3);
    let original_xml = message.to_xml().unwrap();
    let degree = message.body.segment.data.traj[0]
        .interpolation_degree
        .unwrap();
    let invalid = mutated(
        &original_xml,
        &format!("<INTERPOLATION_DEGREE>{degree}</INTERPOLATION_DEGREE>"),
        "<INTERPOLATION_DEGREE>0</INTERPOLATION_DEGREE>",
    );
    assert!(Ocm::from_xml(&invalid).is_err());
    message.body.segment.data.traj[0].interpolation_degree = Some(0);
    assert!(message.validate().is_err());
    assert!(message.to_kvn().is_err());
    assert!(message.to_xml().is_err());
    let mut output = Vec::new();
    assert!(message.write_xml_to(&mut output).is_err());
    assert!(output.is_empty());
    message.body.segment.data.traj[0].interpolation_degree = None;
    // The table's default 3 makes omission usable despite its conditional wording.
    Ocm::from_xml(&message.to_xml().unwrap()).unwrap();

    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.phys = Some(Default::default());
    message
        .body
        .segment
        .data
        .phys
        .as_mut()
        .unwrap()
        .drag_coeff_nom = Some(1.);
    let xml = message.to_xml().unwrap();
    for value in ["0", "-1"] {
        let invalid = mutated(
            &xml,
            "<DRAG_COEFF_NOM>1</DRAG_COEFF_NOM>",
            &format!("<DRAG_COEFF_NOM>{value}</DRAG_COEFF_NOM>"),
        );
        assert!(Ocm::from_xml(&invalid).is_err());
    }
    message
        .body
        .segment
        .data
        .phys
        .as_mut()
        .unwrap()
        .drag_coeff_nom = Some(0.);
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
}

#[test]
fn maneuver_composition_and_unique_epochs_hold_on_public_routes() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.traj.clear();
    message.body.segment.data.man.truncate(1);
    let man = &mut message.body.segment.data.man[0];
    man.man_composition = "TIME_RELATIVE, DEPLOY_ID".into();
    man.man_prev_epoch = None;
    man.man_next_epoch = None;
    man.man_units = None;
    man.man_lines = ["1", "3", "2"]
        .iter()
        .map(|epoch| ccsds_ndm::messages::ocm::ManLine {
            epoch: epoch.parse().unwrap(),
            values: vec!["CHILD=1".into()],
        })
        .collect();
    message.validate().unwrap();
    let kvn = message.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&kvn).unwrap(), message);
    let xml = message.to_xml().unwrap();
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);
    let mut duplicate = message.clone();
    duplicate.body.segment.data.man[0].man_lines[2].epoch = "1.0".parse().unwrap();
    assert_rejects(&duplicate, "MAN data line epoch");
    assert!(Ocm::from_kvn(&kvn.replace("2 CHILD=1", "1.0 CHILD=1")).is_err());
    assert!(Ocm::from_xml(&xml.replace("2 CHILD=1", "1.0 CHILD=1")).is_err());
    // A nonmonotonic sequence that differs beyond f64 precision remains unique.
    message.body.segment.data.man[0].man_lines[2].epoch = "1.0000000000000001".parse().unwrap();
    message.validate().unwrap();
    for composition in [
        "TIME_RELATIVE, UNKNOWN",
        "TIME_RELATIVE, DV_X, DV_X",
        "TIME_RELATIVE, DV_Y, DV_X",
        "TIME_RELATIVE, DEPLOY_ID, THR_X",
    ] {
        let mut invalid = message.clone();
        invalid.body.segment.data.man[0].man_composition = composition.into();
        assert_rejects(&invalid, "MAN_COMPOSITION");
        assert!(Ocm::from_kvn(&kvn.replace("TIME_RELATIVE, DEPLOY_ID", composition)).is_err());
        assert!(Ocm::from_xml(&xml.replace("TIME_RELATIVE, DEPLOY_ID", composition)).is_err());
    }
    // A selected subset may contain only its mandatory time tag.
    let man = &mut message.body.segment.data.man[0];
    man.man_composition = "TIME_RELATIVE".into();
    for line in &mut man.man_lines {
        line.values.clear();
    }
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
}

#[test]
fn orbit_revolution_counts_are_finite_nonnegative_and_may_be_fractional() {
    let message = Ocm::from_kvn(KVN).unwrap();
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut invalid = message.clone();
        invalid.body.segment.data.traj[0].orb_revnum = Some(value);
        assert_rejects(&invalid, "ORB_REVNUM");
    }
    let mut fractional = message;
    fractional.body.segment.data.traj[0].orb_revnum = Some(1.5);
    fractional.body.segment.data.traj[0].orb_revnum_basis = Some(RevNumBasis::One);
    assert_eq!(
        Ocm::from_xml(&fractional.to_xml().unwrap()).unwrap(),
        fractional
    );
    assert_eq!(
        Ocm::from_kvn(&fractional.to_kvn().unwrap()).unwrap(),
        fractional
    );
    let xml = fractional.to_xml().unwrap();
    let kvn = fractional.to_kvn().unwrap();
    for invalid in ["NaN", "INF", "-INF"] {
        assert!(Ocm::from_xml(&mutated(
            &xml,
            "<ORB_REVNUM>1.5</ORB_REVNUM>",
            &format!("<ORB_REVNUM>{invalid}</ORB_REVNUM>")
        ))
        .is_err());
        let invalid_kvn = kvn
            .lines()
            .map(|line| {
                if line.starts_with("ORB_REVNUM ") {
                    format!("ORB_REVNUM = {invalid}")
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_ne!(invalid_kvn, kvn.trim_end());
        assert!(Ocm::from_kvn(&invalid_kvn).is_err());
    }
}

#[test]
fn book_valid_measure_domains_round_trip_kvn_without_xml_facets() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    let od = message.body.segment.data.od.as_mut().unwrap();
    od.days_since_first_obs = Some(ccsds_ndm::types::DayInterval {
        value: -1.0,
        units: None,
    });
    od.days_since_last_obs = Some(ccsds_ndm::types::DayInterval {
        value: -2.0,
        units: None,
    });
    let man = message.body.segment.data.man.first_mut().unwrap();
    man.dc_pa_start_angle = Some(ccsds_ndm::types::Angle {
        value: 400.0,
        units: None,
    });
    man.dc_pa_stop_angle = Some(ccsds_ndm::types::Angle {
        value: -400.0,
        units: None,
    });
    message.validate().unwrap();
    let output = message.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&output).unwrap(), message);
    let mut streamed = Vec::new();
    message.write_kvn_to(&mut streamed).unwrap();
    assert_eq!(streamed, output.as_bytes());
    assert!(message.to_xml().is_err());
}

#[test]
fn omitted_optional_trajectory_defaults_stay_absent() {
    let source = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nTRAJ_START\nCENTER_NAME = EARTH\nTRAJ_REF_FRAME = GCRF\nTRAJ_TYPE = CARTPV\n0 1 2 3 4 5 6\nTRAJ_STOP\n";
    let message = Ocm::from_kvn(source).unwrap();
    let trajectory = &message.body.segment.data.traj[0];
    assert_eq!(trajectory.interpolation_degree, None);
    assert_eq!(trajectory.orb_revnum_basis, None);
    assert_eq!(trajectory.orb_averaging, None);
    let xml = message.to_xml().unwrap();
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    for ending in ["\r", "\r\n", "\n\r"] {
        assert_eq!(
            Ocm::from_kvn(&source.replace('\n', ending)).unwrap(),
            message
        );
    }
}

#[test]
fn user_defined_xml_interleaving_is_preserved_and_kvn_refused() {
    let message = Ocm::from_kvn(include_str!("../../data/kvn/ocm_g15.kvn")).unwrap();
    let user = "<user><COMMENT>before</COMMENT><USER_DEFINED parameter=\"A\">1</USER_DEFINED><COMMENT>between</COMMENT><USER_DEFINED parameter=\"B\">2</USER_DEFINED></user>";
    let xml = message
        .to_xml()
        .unwrap()
        .replace("</data>", &format!("{user}</data>"));
    let parsed = Ocm::from_xml(&xml).unwrap();
    assert!(parsed.to_xml().unwrap().contains(user));
    assert!(parsed.to_kvn().is_err());
    let mut bytes = Vec::new();
    assert!(parsed.write_kvn_to(&mut bytes).is_err());
    assert!(bytes.is_empty());
}

#[test]
fn positive_integer_schema_counts_refuse_zero_in_every_notation() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.od.as_mut().unwrap().consider_n = Some(0);
    message
        .body
        .segment
        .data
        .pert
        .as_mut()
        .unwrap()
        .albedo_grid_size = Some(0);
    message.body.segment.data.man[0].dc_max_cycles = Some(0);
    assert!(message.validate().is_err());
    assert!(message.to_kvn().is_err());
    assert!(message.to_xml().is_err());
    let mut bytes = Vec::new();
    assert!(message.write_xml_to(&mut bytes).is_err());
    assert!(bytes.is_empty());
}

#[test]
fn user_defined_kvn_names_retain_normative_punctuation_and_literal_prefix() {
    let message = Ocm::from_kvn(include_str!("../../data/kvn/ocm_g15.kvn")).unwrap();
    let xml = message.to_xml().unwrap();
    for suffix in ["A.B", "A-B", "USER_DEFINED_LITERAL"] {
        let user =
            format!("<user><USER_DEFINED parameter=\"{suffix}\">value</USER_DEFINED></user>");
        let source = xml.replace("</data>", &format!("{user}</data>"));
        let message = Ocm::from_xml(&source).unwrap();
        assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    }
    let source = xml.replace(
        "</data>",
        "<user><USER_DEFINED parameter=\"lowercase\">value</USER_DEFINED></user></data>",
    );
    assert!(Ocm::from_xml(&source).unwrap().to_kvn().is_err());
}

#[test]
fn required_companion_fields_and_maneuver_domains_are_checked_after_edits() {
    let original = Ocm::from_kvn(KVN).unwrap();
    let mut message = original.clone();
    message.body.segment.data.pert = None;
    assert_rejects(&message, "PERT");
    let mut message = original.clone();
    message.body.segment.data.traj[0].orb_revnum = Some(1.0);
    message.body.segment.data.traj[0].orb_revnum_basis = None;
    assert_rejects(&message, "ORB_REVNUM_BASIS");
    for field in ["OD_ID", "OD_METHOD"] {
        let mut message = original.clone();
        let od = message.body.segment.data.od.as_mut().unwrap();
        if field == "OD_ID" {
            od.od_id = " ".into();
        } else {
            od.od_method = " ".into();
        }
        assert_rejects(&message, field);
    }
    let mut message = original.clone();
    message.body.segment.data.man[0].man_units = Some("[s]".into());
    assert_rejects(&message, "MAN_UNITS");
    let mut message = original.clone();
    let man = &mut message.body.segment.data.man[0];
    let index = man.man_lines[0]
        .values
        .iter()
        .position(|value| value == "ON")
        .unwrap();
    man.man_lines[0].values[index] = "MAYBE".into();
    assert_rejects(&message, "MAN data line THR_INTERP");
    let mut message = original;
    message.body.segment.data.phys.as_mut().unwrap().oeb_qc = Some(-0.5);
    assert_rejects(&message, "OEB_QC");
}

#[test]
fn equinoctial_constant_retrograde_factor_has_no_uncertainty() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    let mut covariance = Ocm::from_kvn(include_str!("../../data/kvn/ocm_g19.kvn"))
        .unwrap()
        .body
        .segment
        .data
        .cov
        .remove(0);
    covariance.traj_id = None;
    covariance.cov_type = "EQUINOCTIAL".into();
    covariance.cov_ordering = ccsds_ndm::types::CovOrder::Ltm;
    covariance.cov_units = None;
    covariance.cov_lines.truncate(1);
    covariance.cov_lines[0].values = (0..7)
        .flat_map(|row| {
            (0..=row).map(move |column| if row == column && row < 6 { 1.0 } else { 0.0 })
        })
        .collect();
    message.body.segment.data.cov = vec![covariance];
    message.body.segment.data.traj.clear();
    message.validate().unwrap();
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
}

#[test]
fn gm_schema_positivity_is_enforced_in_kvn_and_after_model_edits() {
    for value in [0.0, -1.0] {
        let mut message = Ocm::from_kvn(KVN).unwrap();
        message.body.segment.data.pert.as_mut().unwrap().gm =
            Some(ccsds_ndm::types::Gm { value, units: None });
        assert_rejects(&message, "GM");
        let source = mutated(KVN, "PERT_START", &format!("PERT_START\nGM = {value}"));
        assert!(Ocm::from_kvn(&source).is_err());
    }
}

#[test]
fn maneuver_metadata_epochs_are_independent_and_composition_output_is_canonical() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    let man = &mut message.body.segment.data.man[0];
    man.man_prev_epoch = Some("2023-01-01T00:00:00".parse().unwrap());
    man.man_composition = man.man_composition.replace(",", "").to_ascii_lowercase();
    message.validate().unwrap();
    let generated = message.to_kvn().unwrap();
    assert!(generated.lines().any(|line| line
        .split_once('=')
        .is_some_and(|(key, value)| key.trim() == "MAN_COMPOSITION"
            && value.trim().starts_with("TIME_RELATIVE, MAN_DURA, THR_X"))));
    let parsed = Ocm::from_kvn(&generated).unwrap();
    assert_eq!(
        parsed.body.segment.data.man[0]
            .man_prev_epoch
            .as_ref()
            .unwrap()
            .as_str(),
        "2023-01-01T00:00:00"
    );
}
