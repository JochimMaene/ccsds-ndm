use crate::common::{assert_rejects, validate_xml};
use crate::{sample_cdm_kvn, KVN};
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::Ndm;
#[test]
fn edited_shared_od_parameters_are_revalidated_in_cdm() {
    let mut message = Cdm::from_kvn(KVN).unwrap();
    message.body.segments[1]
        .data
        .od_parameters
        .as_mut()
        .unwrap()
        .weighted_rms
        .as_mut()
        .unwrap()
        .value = f64::NAN;

    assert_rejects(&message, "WEIGHTED_RMS");
}

#[test]
fn edited_cdm_numeric_values_are_validated_before_any_output() {
    for field in [
        "MASS",
        "AREA_PC",
        "AREA_DRG",
        "AREA_SRP",
        "CD_AREA_OVER_MASS",
        "CR_AREA_OVER_MASS",
        "SEDR",
        "THRUST_ACCELERATION",
        "COLLISION_PROBABILITY",
    ] {
        for value in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            if field == "THRUST_ACCELERATION" && value == -1.0 {
                continue; // The schema permits signed acceleration.
            }
            let mut message = Cdm::from_kvn(KVN).unwrap();
            let parameters = message.body.segments[0]
                .data
                .additional_parameters
                .as_mut()
                .unwrap();
            parameters.area_drg = parameters.area_pc.clone();
            parameters.area_srp = parameters.area_pc.clone();
            let target = match field {
                "MASS" => &mut parameters.mass.as_mut().unwrap().value,
                "AREA_PC" => &mut parameters.area_pc.as_mut().unwrap().value,
                "AREA_DRG" => &mut parameters.area_drg.as_mut().unwrap().value,
                "AREA_SRP" => &mut parameters.area_srp.as_mut().unwrap().value,
                "CD_AREA_OVER_MASS" => &mut parameters.cd_area_over_mass.as_mut().unwrap().value,
                "CR_AREA_OVER_MASS" => &mut parameters.cr_area_over_mass.as_mut().unwrap().value,
                "SEDR" => &mut parameters.sedr.as_mut().unwrap().value,
                "THRUST_ACCELERATION" => {
                    &mut parameters.thrust_acceleration.as_mut().unwrap().value
                }
                _ => {
                    &mut message
                        .body
                        .relative_metadata_data
                        .collision_probability
                        .as_mut()
                        .unwrap()
                        .value
                }
            };
            *target = value;
            assert_rejects(&message, field);
        }
    }
    for probability in [0.0, 1.0] {
        let mut message = Cdm::from_kvn(KVN).unwrap();
        message
            .body
            .relative_metadata_data
            .collision_probability
            .as_mut()
            .unwrap()
            .value = probability;
        message.body.segments[0]
            .data
            .additional_parameters
            .as_mut()
            .unwrap()
            .mass
            .as_mut()
            .unwrap()
            .value = 0.0;
        message.body.segments[0]
            .data
            .additional_parameters
            .as_mut()
            .unwrap()
            .thrust_acceleration
            .as_mut()
            .unwrap()
            .value = -1.0;
        validate_xml("numeric boundaries", &message.to_xml().unwrap());
        assert_eq!(Cdm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    }
}

#[test]
fn every_kvn_generation_gate_rejects_loss_or_ambiguity_before_output() {
    type CdmMutation = fn(&mut Cdm);
    let cases: [(&str, CdmMutation); 3] = [
        ("first nested logical-block COMMENT", |message| {
            message.body.segments[0]
                .data
                .od_parameters
                .as_mut()
                .unwrap()
                .comment
                .push("nested".to_owned());
        }),
        ("non-printable or non-ASCII character", |message| {
            message.body.segments[0].metadata.object_name = "SATELLITE €".to_owned();
        }),
        ("without changing typed content", |message| {
            message.body.segments[0].metadata.object_name = "SATELLITE\nA".to_owned();
        }),
    ];
    for (label, mutate) in cases {
        let mut message = Cdm::from_kvn(KVN).unwrap();
        mutate(&mut message);
        let error = message.to_kvn().unwrap_err();
        assert!(error.to_string().contains(label), "{label}: {error}");
        let mut output = Vec::new();
        let streamed = message.write_kvn_to(&mut output).unwrap_err();
        assert_eq!(streamed.to_string(), error.to_string(), "{label}");
        assert!(output.is_empty(), "streaming wrote bytes for {label}");
    }
}

#[test]
fn generation_rejects_partial_optional_covariance_rows() {
    let mut message = Cdm::from_kvn(KVN).unwrap();
    let covariance = message.body.segments[0]
        .data
        .covariance_matrix
        .as_mut()
        .unwrap();
    covariance.cdrg_t = None;

    let error = message
        .to_xml()
        .expect_err("a partial optional covariance row violates CCSDS 508.0-B-1 section 5.2.8");
    assert_eq!(error.code(), Some("validation.missing_required_field"));
    assert!(error.to_string().contains("CDRG_T"));
}

/// The relative metadata numbers are plain doubles, so only finiteness constrains them. Before
/// this was routed, `validate` and the XML writer accepted a non-finite value while the KVN
/// writer rejected it, and an infinity reached the document as the lexical `inf`, which `xmllint`
/// rejects as "not a valid value of the atomic type 'xs:double'".
#[test]
fn edited_cdm_relative_metadata_numbers_are_revalidated_before_output() {
    type Mutation = fn(&mut Cdm, f64);
    let cases: [(&str, Mutation); 5] = [
        ("MISS_DISTANCE", |cdm, value| {
            cdm.body.relative_metadata_data.miss_distance.value = value
        }),
        ("RELATIVE_SPEED", |cdm, value| {
            cdm.body
                .relative_metadata_data
                .relative_speed
                .as_mut()
                .expect("fixture has RELATIVE_SPEED")
                .value = value
        }),
        ("RELATIVE_POSITION_T", |cdm, value| {
            cdm.body
                .relative_metadata_data
                .relative_state_vector
                .as_mut()
                .expect("fixture has a relative state vector")
                .relative_position_t
                .value = value
        }),
        ("RELATIVE_VELOCITY_N", |cdm, value| {
            cdm.body
                .relative_metadata_data
                .relative_state_vector
                .as_mut()
                .expect("fixture has a relative state vector")
                .relative_velocity_n
                .value = value
        }),
        ("SCREEN_VOLUME_Y", |cdm, value| {
            cdm.body
                .relative_metadata_data
                .screen_volume_y
                .as_mut()
                .expect("fixture has SCREEN_VOLUME_Y")
                .value = value
        }),
    ];

    for (field, mutate) in cases {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut cdm = Cdm::from_kvn(KVN).expect("fixture should parse");
            mutate(&mut cdm, value);

            assert_rejects(&cdm, field);
        }
    }

    // A finite replacement still reaches the reference schema.
    let mut cdm = Cdm::from_kvn(KVN).expect("fixture should parse");
    cdm.body.relative_metadata_data.miss_distance.value = 0.0;
    cdm.validate().expect("zero miss distance is representable");
    validate_xml("CDM relative metadata boundary", &cdm.to_xml().unwrap());
}

#[test]
fn cdm_kvn_comments_keep_their_normative_block_association() {
    let cdm = Cdm::from_kvn(KVN).unwrap();

    assert_eq!(
        cdm.body.relative_metadata_data.comment,
        ["Relative Metadata/Data"]
    );

    let first = &cdm.body.segments[0];
    assert_eq!(first.metadata.comment, ["Object1 Metadata"]);
    // KVN has no delimiter between the outer data comments and the first nested block's
    // comments. Preserve the leading run and its position without guessing a split.
    assert_eq!(
        first.data.comment,
        ["Object1 Data", "Object1 OD Parameters"]
    );
    assert!(first
        .data
        .od_parameters
        .as_ref()
        .unwrap()
        .comment
        .is_empty());
    assert_eq!(
        first.data.additional_parameters.as_ref().unwrap().comment,
        [
            "Object1 Additional Parameters",
            "Apogee Altitude=779 km",
            "Perigee Altitude=765 km",
            "Inclination=86.4 deg",
        ]
    );
    assert_eq!(first.data.state_vector.comment, ["Object1 State Vector"]);
    assert_eq!(
        first.data.covariance_matrix.as_ref().unwrap().comment,
        ["Object1 Covariance in the RTN Coordinate Frame"]
    );

    let regenerated = cdm.to_kvn().unwrap();
    assert_eq!(Cdm::from_kvn(&regenerated).unwrap(), cdm);
}

#[test]
fn kvn_roundtrip() {
    let kvn = sample_cdm_kvn();
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    let regenerated = cdm.to_kvn().expect("to_kvn");
    // Parse again to ensure structural equality
    let cdm2 = Cdm::from_kvn(&regenerated).expect("re-parse");
    assert_eq!(cdm2, cdm);
}

#[test]
fn xml_roundtrip() {
    let kvn = sample_cdm_kvn();
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    let xml = cdm.to_xml().expect("to_xml");
    let cdm2 = Cdm::from_xml(&xml).expect("from_xml");
    assert_eq!(cdm2, cdm);
}

#[test]
fn full_optional_fields_roundtrip() {
    let kvn = r#"
CCSDS_CDM_VERS = 1.0
COMMENT Header comment
CREATION_DATE = 2025-01-01T00:00:00

ORIGINATOR = TEST
MESSAGE_FOR = RECIPIENT
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
RELATIVE_VELOCITY_N = 0.05 [m/s]
START_SCREEN_PERIOD = 2025-01-02T11:00:00
STOP_SCREEN_PERIOD = 2025-01-02T13:00:00
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
SCREEN_ENTRY_TIME = 2025-01-02T11:30:00
SCREEN_EXIT_TIME = 2025-01-02T12:30:00
COLLISION_PROBABILITY = 0.001
COLLISION_PROBABILITY_METHOD = FOSTER-1992

COMMENT Segment 1
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
OPERATOR_CONTACT_POSITION = POSITION
OPERATOR_ORGANIZATION = ORG
OPERATOR_PHONE = PHONE
OPERATOR_EMAIL = EMAIL
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
ORBIT_CENTER = EARTH
REF_FRAME = EME2000
GRAVITY_MODEL = EGM-96
ATMOSPHERIC_MODEL = JACCHIA 70 DCA
N_BODY_PERTURBATIONS = MOON,SUN
SOLAR_RAD_PRESSURE = YES
EARTH_TIDES = YES
INTRACK_THRUST = YES

TIME_LASTOB_START = 2025-01-01T00:00:00
TIME_LASTOB_END = 2025-01-02T00:00:00
RECOMMENDED_OD_SPAN = 7.0 [d]
ACTUAL_OD_SPAN = 5.0 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
RESIDUALS_ACCEPTED = 95.5 [%]
WEIGHTED_RMS = 1.23

AREA_PC = 10.0 [m**2]
AREA_DRG = 12.0 [m**2]
AREA_SRP = 15.0 [m**2]
MASS = 1000.0 [kg]
CD_AREA_OVER_MASS = 0.012 [m**2/kg]
CR_AREA_OVER_MASS = 0.015 [m**2/kg]
THRUST_ACCELERATION = 0.001 [m/s**2]
SEDR = 0.05 [W/kg]

X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.1 [km/s]
Y_DOT = 0.2 [km/s]
Z_DOT = 0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
CDRG_R = 0.001 [m**3/kg]
CDRG_T = 0.002 [m**3/kg]
CDRG_N = 0.003 [m**3/kg]
CDRG_RDOT = 0.0001 [m**3/(kg*s)]
CDRG_TDOT = 0.0002 [m**3/(kg*s)]
CDRG_NDOT = 0.0003 [m**3/(kg*s)]
CDRG_DRG = 0.00001 [m**4/kg**2]
CSRP_R = 0.001 [m**3/kg]
CSRP_T = 0.002 [m**3/kg]
CSRP_N = 0.003 [m**3/kg]
CSRP_RDOT = 0.0001 [m**3/(kg*s)]
CSRP_TDOT = 0.0002 [m**3/(kg*s)]
CSRP_NDOT = 0.0003 [m**3/(kg*s)]
CSRP_DRG = 0.00001 [m**4/kg**2]
CSRP_SRP = 0.00002 [m**4/kg**2]
CTHR_R = 0.001 [m**2/s**2]
CTHR_T = 0.002 [m**2/s**2]
CTHR_N = 0.003 [m**2/s**2]
CTHR_RDOT = 0.0001 [m**2/s**3]
CTHR_TDOT = 0.0002 [m**2/s**3]
CTHR_NDOT = 0.0003 [m**2/s**3]
CTHR_DRG = 0.00001 [m**3/(kg*s**2)]
CTHR_SRP = 0.00002 [m**3/(kg*s**2)]
CTHR_THR = 0.000001 [m**2/s**4]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 00002
CATALOG_NAME = CAT
OBJECT_NAME = OBJ2
INTERNATIONAL_DESIGNATOR = 1998-067B
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000

X = -1.0 [km]
Y = -2.0 [km]
Z = -3.0 [km]
X_DOT = -0.1 [km/s]
Y_DOT = -0.2 [km/s]
Z_DOT = -0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"#;
    let cdm = Cdm::from_kvn(kvn).expect("parse full cdm");
    let regenerated = cdm.to_kvn().expect("generate full kvn");
    let cdm2 = Cdm::from_kvn(&regenerated).expect("parse regenerated full cdm");

    assert!(cdm.body.segments[0].data.od_parameters.is_some());
    assert!(cdm.body.segments[0]
        .data
        .covariance_matrix
        .as_ref()
        .unwrap()
        .cthr_thr
        .is_some());
    assert_eq!(cdm2, cdm);
}
