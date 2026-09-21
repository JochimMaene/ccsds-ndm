use crate::common::{assert_validation_field, mutated};
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::types::CdmObjectType;
use ccsds_ndm::Ndm;
fn sample_cdm_kvn() -> String {
    let kvn = r#"
CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = OPERATOR
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
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
COLLISION_PROBABILITY = 0.001
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000

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
    kvn.to_string()
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

#[test]
fn cdm_validation_probability_range() {
    // Probability > 1.0
    let mut kvn = sample_cdm_kvn();
    kvn = mutated(
        &kvn,
        "COLLISION_PROBABILITY = 0.001",
        "COLLISION_PROBABILITY = 1.5",
    );
    crate::common::assert_validation_field(&Cdm::from_kvn(&kvn).unwrap_err(), "Probability");
}

#[test]
fn cdm_validation_segment_count_mismatch() {
    let mut cdm = Cdm::from_kvn(&sample_cdm_kvn()).unwrap();

    // Remove one segment => 1 segment
    cdm.body.segments.pop();
    assert_validation_field(&cdm.validate().unwrap_err(), "exactly 2 segments");

    // Add valid segments to check 3 segments (also invalid)
    let seg = cdm.body.segments[0].clone();
    cdm.body.segments.push(seg.clone());
    cdm.body.segments.push(seg); // Now 3
    assert_eq!(cdm.body.segments.len(), 3);
    assert_validation_field(&cdm.validate().unwrap_err(), "exactly 2 segments");
}

#[test]
fn cdm_validation_requires_object1_and_object2() {
    let mut cdm = Cdm::from_kvn(&sample_cdm_kvn()).unwrap();
    cdm.body.segments[1].metadata.object = CdmObjectType::Object1;
    assert_validation_field(
        &cdm.validate().unwrap_err(),
        "exactly one OBJECT1 and one OBJECT2",
    );
}
