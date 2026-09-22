use crate::common::{assert_rejects, mutated, validate_xml};
use crate::KVN;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::types::{
    Angle, Area, DayInterval, Duration, Gm, ManDc, Mass, NonNegativeDouble, ObjectDescription,
    Percentage, Probability, RevNumBasis, TrajBasis, Vec3Double,
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
    assert_rejects(&message, "TRAJ trajLine 2 value 3");
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
        assert_rejects(&message, "manLine THR_X");
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
    assert_rejects(&message, "manLine THR_X");
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
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
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
COV_UNITS = km**2
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
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
    assert_eq!(cov.cov_units, Some("km**2".to_string()));

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
    assert_eq!(meta.object_type, Some(ObjectDescription::Payload));
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
TRAJ_UNITS = km km/s
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let traj = &ocm.body.segment.data.traj[0];
    assert_eq!(traj.traj_id, Some("T1".to_string()));
    assert_eq!(traj.traj_basis, Some(TrajBasis::Predicted));
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
