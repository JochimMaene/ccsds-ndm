// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::from_str;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
struct CaseEntry {
    id: String,
    #[serde(rename = "expected")]
    _expected: String,
    #[serde(rename = "category")]
    _category: String,
    behavior: String,
}

#[test]
fn test_case_matrix_mapped() {
    let cases = load_cases();
    let mut unmapped = Vec::new();

    for case in &cases {
        if !behavior_catalog().contains_key(case.behavior.as_str()) {
            unmapped.push(case.id.clone());
        }
    }

    assert!(
        unmapped.is_empty(),
        "Unmapped cases ({}):\n{}",
        unmapped.len(),
        unmapped.join("\n")
    );
}

#[test]
fn test_case_matrix_behavior_catalog() {
    let cases = load_cases();
    let mut used = HashSet::new();
    for case in &cases {
        used.insert(case.behavior.as_str());
    }

    let catalog = behavior_catalog();
    let missing: Vec<_> = used
        .iter()
        .filter(|id| !catalog.contains_key(**id))
        .copied()
        .collect();

    assert!(
        missing.is_empty(),
        "Behaviors missing minimal test fixtures: {missing:?}"
    );
}

#[test]
fn test_minimal_success_cases() {
    for (behavior, minimal) in behavior_catalog() {
        if !minimal.should_parse {
            continue;
        }
        let res = from_str(&minimal.input);
        assert!(
            res.is_ok(),
            "Expected success for {behavior}, got error: {res:?}"
        );
    }
}

#[test]
fn test_minimal_failure_cases() {
    for (behavior, minimal) in behavior_catalog() {
        if minimal.should_parse {
            continue;
        }
        let res = from_str(&minimal.input);
        assert!(res.is_err(), "Expected failure for {behavior}, got success");
    }
}

struct MinimalCase {
    input: String,
    should_parse: bool,
}

fn behavior_catalog() -> HashMap<&'static str, MinimalCase> {
    let mut map = HashMap::new();

    // Success behaviors (minimal valid KVN/XML per message type)
    map.insert(
        "acm_success_kvn",
        MinimalCase {
            input: minimal_acm_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "aem_success_kvn",
        MinimalCase {
            input: minimal_aem_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "apm_success_kvn",
        MinimalCase {
            input: minimal_apm_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "cdm_success_kvn",
        MinimalCase {
            input: minimal_cdm_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "ocm_success_kvn",
        MinimalCase {
            input: minimal_ocm_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "oem_success_kvn",
        MinimalCase {
            input: minimal_oem_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "oem_success_xml",
        MinimalCase {
            input: minimal_oem_xml(),
            should_parse: true,
        },
    );
    map.insert(
        "omm_success_kvn",
        MinimalCase {
            input: minimal_omm_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "opm_success_kvn",
        MinimalCase {
            input: minimal_opm_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "tdm_success_kvn",
        MinimalCase {
            input: minimal_tdm_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "tdm_success_xml",
        MinimalCase {
            input: minimal_tdm_xml(),
            should_parse: true,
        },
    );
    map.insert(
        "rdm_success_kvn",
        MinimalCase {
            input: minimal_rdm_kvn(),
            should_parse: true,
        },
    );
    map.insert(
        "ndm_success_xml",
        MinimalCase {
            input: minimal_ndm_xml(),
            should_parse: true,
        },
    );

    // Generic failure behaviors per message class.
    map.insert(
        "acm_missing",
        MinimalCase {
            input: acm_missing_cp_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "acm_spurious",
        MinimalCase {
            input: acm_spurious_metadata_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "acm_number_format",
        MinimalCase {
            input: acm_number_format_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "acm_duplicate",
        MinimalCase {
            input: acm_duplicate_sensor_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "acm_inconsistent",
        MinimalCase {
            input: acm_conflict_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "aem_missing",
        MinimalCase {
            input: aem_missing_attitude_type_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "aem_wrong_keyword",
        MinimalCase {
            input: aem_wrong_keyword_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "aem_number_format",
        MinimalCase {
            input: aem_number_format_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "aem_inconsistent",
        MinimalCase {
            input: aem_inconsistent_data_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "apm_missing",
        MinimalCase {
            input: apm_missing_frame_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "apm_wrong_keyword",
        MinimalCase {
            input: apm_wrong_keyword_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "apm_number_format",
        MinimalCase {
            input: apm_number_format_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "apm_invalid_value",
        MinimalCase {
            input: apm_invalid_euler_seq_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "cdm_missing",
        MinimalCase {
            input: cdm_missing_tca_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "cdm_wrong_keyword",
        MinimalCase {
            input: cdm_wrong_keyword_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "cdm_number_format",
        MinimalCase {
            input: cdm_number_format_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "cdm_invalid_value",
        MinimalCase {
            input: cdm_invalid_tca_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "ocm_missing",
        MinimalCase {
            input: ocm_missing_tzero_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "ocm_wrong_keyword",
        MinimalCase {
            input: ocm_wrong_keyword_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "ocm_invalid_value",
        MinimalCase {
            input: ocm_unknown_frame_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "ocm_invalid_units",
        MinimalCase {
            input: ocm_invalid_units_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "oem_number_format",
        MinimalCase {
            input: oem_number_format_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "oem_wrong_keyword",
        MinimalCase {
            input: oem_wrong_keyword_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "oem_inconsistent",
        MinimalCase {
            input: oem_inconsistent_time_system_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "omm_missing",
        MinimalCase {
            input: omm_missing_object_id_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "omm_wrong_keyword",
        MinimalCase {
            input: omm_wrong_keyword_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "omm_number_format",
        MinimalCase {
            input: omm_number_format_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "opm_invalid_units",
        MinimalCase {
            input: opm_invalid_units_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "opm_invalid_value",
        MinimalCase {
            input: opm_invalid_units_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "opm_wrong_keyword",
        MinimalCase {
            input: opm_wrong_keyword_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "opm_number_format",
        MinimalCase {
            input: opm_number_format_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "tdm_missing",
        MinimalCase {
            input: tdm_missing_time_system_xml(),
            should_parse: false,
        },
    );
    map.insert(
        "tdm_wrong_keyword",
        MinimalCase {
            input: tdm_wrong_keyword_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "tdm_number_format",
        MinimalCase {
            input: tdm_number_format_kvn(),
            should_parse: false,
        },
    );
    map.insert(
        "tdm_inconsistent",
        MinimalCase {
            input: tdm_inconsistent_time_system_kvn(),
            should_parse: false,
        },
    );

    map.insert(
        "ndm_wrong_format",
        MinimalCase {
            input: "NOT_A_CCSDS_MESSAGE".to_string(),
            should_parse: false,
        },
    );

    map
}

fn load_cases() -> Vec<CaseEntry> {
    let raw = include_str!("case_matrix.json");
    serde_json::from_str(raw).expect("case_matrix.json must be valid JSON")
}

// ---------------- Minimal inputs ----------------

fn minimal_acm_kvn() -> String {
    r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_DESIGNATOR = 2020-001A
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
ATT_TYPE = QUATERNION
NUMBER_STATES = 4
0.0 0 0 0 1
ATT_STOP
"#
    .to_string()
}

fn acm_missing_cp_kvn() -> String {
    let mut s = minimal_acm_kvn();
    s.push_str("PHYS_START\nCP_REF_FRAME = SC_BODY\nPHYS_STOP\n");
    s
}

fn acm_spurious_metadata_kvn() -> String {
    let mut s = minimal_acm_kvn();
    s = s.replace("META_STOP", "BAD_KEY = 1\nMETA_STOP");
    s
}

fn acm_number_format_kvn() -> String {
    let mut s = minimal_acm_kvn();
    s = s.replace("0.0 0 0 0 1", "NOT_A_DATE 0 0 0 1");
    s
}

fn acm_duplicate_sensor_kvn() -> String {
    let mut s = minimal_acm_kvn();
    s.push_str("AD_START\nSENSOR_NUMBER = 1\nAD_STOP\nAD_START\nSENSOR_NUMBER = 1\nAD_STOP\n");
    s
}

fn acm_conflict_kvn() -> String {
    let mut s = minimal_acm_kvn();
    s.push_str("MAN_START\nMAN_END_TIME = 2023-01-01T00:00:10\nMAN_DURATION = 10\nMAN_STOP\n");
    s
}

fn minimal_aem_kvn() -> String {
    r#"CCSDS_AEM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T00:10:00
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#
    .to_string()
}

fn aem_missing_attitude_type_kvn() -> String {
    minimal_aem_kvn().replace("ATTITUDE_TYPE = QUATERNION\n", "")
}

fn aem_wrong_keyword_kvn() -> String {
    minimal_aem_kvn().replace("META_STOP", "BAD_KEY = 1\nMETA_STOP")
}

fn aem_number_format_kvn() -> String {
    minimal_aem_kvn().replace("0 0 0 1", "0 0 BAD 1")
}

fn aem_inconsistent_data_kvn() -> String {
    minimal_aem_kvn().replace("0 0 0 1", "0 0 0")
}

fn minimal_apm_kvn() -> String {
    r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
META_STOP
EPOCH = 2023-01-01T00:00:00
QUAT_START
REF_FRAME_A = SC_BODY_1
REF_FRAME_B = GCRF
Q1 = 0
Q2 = 0
Q3 = 0
QC = 1
QUAT_STOP
"#
    .to_string()
}

fn apm_missing_frame_kvn() -> String {
    minimal_apm_kvn().replace("REF_FRAME_A = SC_BODY_1\n", "")
}

fn apm_wrong_keyword_kvn() -> String {
    minimal_apm_kvn().replace("META_STOP", "BAD_KEY = 1\nMETA_STOP")
}

fn apm_number_format_kvn() -> String {
    minimal_apm_kvn().replace("Q1 = 0", "Q1 = BAD")
}

fn apm_invalid_euler_seq_kvn() -> String {
    let mut s = minimal_apm_kvn();
    s.push_str(
        "EULER_START\nREF_FRAME_A = SC_BODY_1\nREF_FRAME_B = GCRF\nEULER_ROT_SEQ = BAD\nANGLE_1 = 0\nANGLE_2 = 0\nANGLE_3 = 0\nEULER_STOP\n",
    );
    s
}

fn minimal_cdm_kvn() -> String {
    r#"CCSDS_CDM_VERS = 1.0
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
"#
    .to_string()
}

fn cdm_missing_tca_kvn() -> String {
    minimal_cdm_kvn().replace("TCA = 2025-01-02T12:00:00\n", "")
}

fn cdm_wrong_keyword_kvn() -> String {
    minimal_cdm_kvn().replace("MISS_DISTANCE", "BAD_KEY")
}

fn cdm_number_format_kvn() -> String {
    minimal_cdm_kvn().replace("MISS_DISTANCE = 100.0", "MISS_DISTANCE = BAD")
}

fn cdm_invalid_tca_kvn() -> String {
    minimal_cdm_kvn().replace("TCA = 2025-01-02T12:00:00", "TCA = NOT_A_DATE")
}

fn minimal_ocm_kvn() -> String {
    r#"CCSDS_OCM_VERS = 3.0
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
"#
    .to_string()
}

fn ocm_missing_tzero_kvn() -> String {
    minimal_ocm_kvn().replace("EPOCH_TZERO = 2023-01-01T00:00:00\n", "")
}

fn ocm_wrong_keyword_kvn() -> String {
    minimal_ocm_kvn().replace("TIME_SYSTEM", "BAD_KEY")
}

fn ocm_unknown_frame_kvn() -> String {
    minimal_ocm_kvn().replace(
        "TRAJ_REF_FRAME = GCRF",
        "TRAJ_REF_FRAME = GCRF\nTRAJ_BASIS = INVALID",
    )
}

fn ocm_invalid_units_kvn() -> String {
    minimal_ocm_kvn().replace(
        "TRAJ_TYPE = CARTPV",
        "TRAJ_TYPE = CARTPV\nTRAJ_UNITS = [m, m, m, m/s, m/s, m/s]",
    )
}

fn minimal_oem_kvn() -> String {
    r#"CCSDS_OEM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T00:10:00
META_STOP
2023-01-01T00:00:00 1 2 3 4 5 6
"#
    .to_string()
}

fn oem_number_format_kvn() -> String {
    minimal_oem_kvn().replace("1 2 3 4 5 6", "1 2 BAD 4 5 6")
}

fn oem_wrong_keyword_kvn() -> String {
    minimal_oem_kvn().replace("REF_FRAME", "BAD_KEY")
}

fn oem_inconsistent_time_system_kvn() -> String {
    let mut s = minimal_oem_kvn();
    s.push_str(
        "META_START\nOBJECT_NAME = TEST2\nOBJECT_ID = 2020-002A\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\nTIME_SYSTEM = TAI\nSTART_TIME = 2023-01-01T00:10:00\nSTOP_TIME = 2023-01-01T00:20:00\nMETA_STOP\nDATA_START\n2023-01-01T00:10:00 1 2 3 4 5 6\nDATA_STOP\n",
    );
    s
}

fn minimal_omm_kvn() -> String {
    r#"CCSDS_OMM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0
ECCENTRICITY = 0.001
INCLINATION = 98.0
RA_OF_ASC_NODE = 10.0
ARG_OF_PERICENTER = 20.0
MEAN_ANOMALY = 30.0
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
BSTAR = 0.0001 [1/ER]
"#
    .to_string()
}

fn omm_missing_object_id_kvn() -> String {
    minimal_omm_kvn().replace("OBJECT_ID = 2020-001A\n", "")
}

fn omm_wrong_keyword_kvn() -> String {
    minimal_omm_kvn().replace("MEAN_ELEMENT_THEORY", "BAD_KEY")
}

fn omm_number_format_kvn() -> String {
    minimal_omm_kvn().replace("MEAN_MOTION = 15.0", "MEAN_MOTION = BAD")
}

fn minimal_opm_kvn() -> String {
    r#"CCSDS_OPM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 1 [km]
Y = 2 [km]
Z = 3 [km]
X_DOT = 4 [km/s]
Y_DOT = 5 [km/s]
Z_DOT = 6 [km/s]
"#
    .to_string()
}

fn opm_invalid_units_kvn() -> String {
    minimal_opm_kvn().replace("X = 1 [km]", "X = 1 [m]")
}

fn opm_wrong_keyword_kvn() -> String {
    minimal_opm_kvn().replace("REF_FRAME", "BAD_KEY")
}

fn opm_number_format_kvn() -> String {
    minimal_opm_kvn().replace("X = 1 [km]", "X = BAD [km]")
}

fn minimal_tdm_kvn() -> String {
    r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = STATION
PARTICIPANT_2 = SPACECRAFT
PATH_1 = 1,2
PATH_2 = 2,1
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 100
DATA_STOP
"#
    .to_string()
}

fn minimal_tdm_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<tdm id="CCSDS_TDM_VERS" version="2.0">
  <header>
    <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
    <ORIGINATOR>TEST</ORIGINATOR>
  </header>
  <body>
    <segment>
      <metadata>
        <TIME_SYSTEM>UTC</TIME_SYSTEM>
        <PARTICIPANT_1>STATION</PARTICIPANT_1>
        <PARTICIPANT_2>SPACECRAFT</PARTICIPANT_2>
        <PATH_1>1,2</PATH_1>
        <PATH_2>2,1</PATH_2>
      </metadata>
      <data>
        <observation>
          <EPOCH>2023-01-01T00:00:00</EPOCH>
          <RANGE>100</RANGE>
        </observation>
      </data>
    </segment>
  </body>
</tdm>
"#
    .to_string()
}

fn tdm_wrong_keyword_kvn() -> String {
    minimal_tdm_kvn().replace("TIME_SYSTEM", "BAD_KEY")
}

fn tdm_number_format_kvn() -> String {
    minimal_tdm_kvn().replace(
        "RANGE = 2023-01-01T00:00:00 100",
        "RANGE = 2023-01-01T00:00:00 BAD",
    )
}

fn tdm_inconsistent_time_system_kvn() -> String {
    minimal_tdm_kvn().replace("PATH_2 = 2,1\n", "")
}

fn tdm_missing_time_system_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<tdm id="CCSDS_TDM_VERS" version="2.0">
  <header>
    <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
    <ORIGINATOR>TEST</ORIGINATOR>
  </header>
  <body>
    <segment>
      <metadata>
        <PARTICIPANT_1>STATION</PARTICIPANT_1>
      </metadata>
      <data>
        <observation>
          <EPOCH>2023-01-01T00:00:00</EPOCH>
          <RANGE>100</RANGE>
        </observation>
      </data>
    </segment>
  </body>
</tdm>
"#
    .to_string()
}

fn minimal_rdm_kvn() -> String {
    r#"CCSDS_RDM_VERS = 1.0
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
"#
    .to_string()
}

fn minimal_ndm_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<ndm>
  <opm id="CCSDS_OPM_VERS" version="2.0">
    <header>
      <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
      <ORIGINATOR>TEST</ORIGINATOR>
    </header>
    <body>
      <segment>
        <metadata>
          <OBJECT_NAME>TEST</OBJECT_NAME>
          <OBJECT_ID>2020-001A</OBJECT_ID>
          <CENTER_NAME>EARTH</CENTER_NAME>
          <REF_FRAME>GCRF</REF_FRAME>
          <TIME_SYSTEM>UTC</TIME_SYSTEM>
        </metadata>
        <data>
          <stateVector>
            <EPOCH>2023-01-01T00:00:00</EPOCH>
            <X units="km">1</X>
            <Y units="km">2</Y>
            <Z units="km">3</Z>
            <X_DOT units="km/s">4</X_DOT>
            <Y_DOT units="km/s">5</Y_DOT>
            <Z_DOT units="km/s">6</Z_DOT>
          </stateVector>
        </data>
      </segment>
    </body>
  </opm>
</ndm>
"#
    .to_string()
}

fn minimal_oem_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<oem id="CCSDS_OEM_VERS" version="2.0">
  <header>
    <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
    <ORIGINATOR>TEST</ORIGINATOR>
  </header>
  <body>
    <segment>
      <metadata>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2020-001A</OBJECT_ID>
        <CENTER_NAME>EARTH</CENTER_NAME>
        <REF_FRAME>GCRF</REF_FRAME>
        <TIME_SYSTEM>UTC</TIME_SYSTEM>
        <START_TIME>2023-01-01T00:00:00</START_TIME>
        <STOP_TIME>2023-01-01T00:10:00</STOP_TIME>
      </metadata>
      <data>
        <stateVector>
          <EPOCH>2023-01-01T00:00:00</EPOCH>
          <X units="km">1</X>
          <Y units="km">2</Y>
          <Z units="km">3</Z>
          <X_DOT units="km/s">4</X_DOT>
          <Y_DOT units="km/s">5</Y_DOT>
          <Z_DOT units="km/s">6</Z_DOT>
        </stateVector>
      </data>
    </segment>
  </body>
</oem>
"#
    .to_string()
}
