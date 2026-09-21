use crate::common::mutated;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::Ndm;
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

/// Parse official RDM XML example C-3 (minimal)
#[test]
fn rdm_sample_c3_xml() {
    let xml = include_str!("../../data/xml/rdm_c3.xml");
    let rdm = Rdm::from_xml(xml).unwrap();
    assert_eq!(rdm.version, "1.0");
    assert_eq!(rdm.header.originator, "ESA");
    assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
}

/// Parse official RDM XML example C-4 (comprehensive)
#[test]
fn rdm_sample_c4_xml() {
    let xml = include_str!("../../data/xml/rdm_c4.xml");
    let rdm = Rdm::from_xml(xml).unwrap();
    assert_eq!(rdm.header.message_id, "ESA/20180422-001");
    assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
    assert!(rdm.body.segment.data.state_vector.is_some());
    assert!(rdm.body.segment.data.covariance_matrix.is_some());
    assert!(rdm.body.segment.data.spacecraft_parameters.is_some());
    assert!(rdm.body.segment.data.od_parameters.is_some());
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

/// The conditional RDM rules, as single mutations of a shipped fixture.
#[test]
fn rdm_conditional_rules_name_the_offending_field() {
    const FIXTURE: &str = include_str!("../../data/kvn/rdm_c2.kvn");
    const STATE_VECTOR: &str = "COMMENT State vector at the last OD epoch";
    const COVARIANCE: &str = "COMMENT Position/velocity covariance matrix at last OD epoch";
    Rdm::from_kvn(FIXTURE).expect("baseline fixture must satisfy every rule");

    let state_vector_at = FIXTURE.find(STATE_VECTOR).unwrap();
    let covariance_at = FIXTURE.find(COVARIANCE).unwrap();
    // Neither block is mandatory, so the truncation is a valid message in its own right.
    let no_state_or_covariance = &FIXTURE[..state_vector_at];
    Rdm::from_kvn(no_state_or_covariance)
        .expect("a message with neither optional block must still parse");
    let covariance_without_state =
        format!("{}{}", no_state_or_covariance, &FIXTURE[covariance_at..]);

    for (mutated, expected) in [
            (
                mutated(FIXTURE, "REENTRY_ALTITUDE = 80.0 [km]",
                    "REENTRY_ALTITUDE = 80.0 [km]\nORBIT_LIFETIME_WINDOW_START = 6.0 [d]\nORBIT_LIFETIME_WINDOW_END = 5.0 [d]",
                ),
                "ORBIT_LIFETIME_WINDOW_START must be <= ORBIT_LIFETIME_WINDOW_END",
            ),
            (
                mutated(FIXTURE, "REF_FRAME = EME2000\n", ""),
                "Missing required field: REF_FRAME (required when state vector is provided)",
            ),
            (
                covariance_without_state,
                "Missing required field: stateVector (required when covarianceMatrix is provided)",
            ),
            (
                mutated(FIXTURE, "OBJECT_NAME = SPACEOBJECT", "OBJECT_NAME ="),
                "Missing required field: OBJECT_NAME",
            ),
        ] {
            let error = Rdm::from_kvn(&mutated).expect_err("mutation accepted");
            assert!(
                error.to_string().contains(expected),
                "diagnostic did not name {expected}: {error}"
            );
        }
}
