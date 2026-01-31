// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::from_str;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn test_parse_minimal_fixtures() {
    let cases = minimal_cases();

    let mut failures = Vec::new();
    for case in cases {
        let res = from_str(case.input);
        if case.should_parse {
            match res {
                Ok(msg) => {
                    // round-trip
                    if case.is_xml {
                        if let Ok(xml_out) = msg.to_xml() {
                            if let Err(e) = from_str(&xml_out) {
                                failures
                                    .push(format!("{} XML round-trip failed: {}", case.name, e));
                            }
                        } else {
                            failures.push(format!("{} failed to serialize to XML", case.name));
                        }
                    } else {
                        if let Ok(kvn_out) = msg.to_kvn() {
                            if let Err(e) = from_str(&kvn_out) {
                                failures
                                    .push(format!("{} KVN round-trip failed: {}", case.name, e));
                            }
                        } else {
                            failures.push(format!("{} failed to serialize to KVN", case.name));
                        }
                    }
                }
                Err(e) => failures.push(format!("{} failed to parse: {}", case.name, e)),
            }
        } else if res.is_ok() {
            failures.push(format!("{} parsed but was expected to fail", case.name));
        }
    }

    if !failures.is_empty() {
        panic!(
            "Encountered {} parsing failures:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}

#[test]
fn test_parse_data_samples() {
    let data_root = data_dir();
    let mut failures = Vec::new();

    for file in sorted_files(&data_root.join("kvn"), "kvn") {
        let content = match fs::read_to_string(&file) {
            Ok(content) => content,
            Err(e) => {
                failures.push(format!("{} failed to read: {}", file.display(), e));
                continue;
            }
        };

        if let Err(e) = from_str(&content) {
            failures.push(format!("{} failed to parse: {}", file.display(), e));
        }
    }

    for file in sorted_files(&data_root.join("xml"), "xml") {
        let content = match fs::read_to_string(&file) {
            Ok(content) => content,
            Err(e) => {
                failures.push(format!("{} failed to read: {}", file.display(), e));
                continue;
            }
        };

        if let Err(e) = from_str(&content) {
            failures.push(format!("{} failed to parse: {}", file.display(), e));
        }
    }

    if !failures.is_empty() {
        panic!(
            "Encountered {} data sample failures:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}

struct MinimalCase<'a> {
    name: &'a str,
    input: &'a str,
    should_parse: bool,
    is_xml: bool,
}

fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("data")
}

fn sorted_files(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => panic!("Failed to read data directory {}: {}", dir.display(), e),
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                panic!("Failed to read entry in {}: {}", dir.display(), e);
            }
        };
        let path = entry.path();
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case(extension))
            .unwrap_or(false)
        {
            files.push(path);
        }
    }

    files.sort();
    files
}

fn minimal_cases<'a>() -> Vec<MinimalCase<'a>> {
    vec![
        MinimalCase {
            name: "acm_minimal_kvn",
            input: ACM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "aem_minimal_kvn",
            input: AEM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "apm_minimal_kvn",
            input: APM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "cdm_minimal_kvn",
            input: CDM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "ocm_minimal_kvn",
            input: OCM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "oem_minimal_kvn",
            input: OEM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "omm_minimal_kvn",
            input: OMM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "opm_minimal_kvn",
            input: OPM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "tdm_minimal_kvn",
            input: TDM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "rdm_minimal_kvn",
            input: RDM_MINIMAL_KVN,
            should_parse: true,
            is_xml: false,
        },
        MinimalCase {
            name: "ndm_minimal_xml",
            input: NDM_MINIMAL_XML,
            should_parse: true,
            is_xml: true,
        },
        MinimalCase {
            name: "oem_minimal_xml",
            input: OEM_MINIMAL_XML,
            should_parse: true,
            is_xml: true,
        },
        // a few negative cases to ensure error paths are exercised at integration level
        MinimalCase {
            name: "tdm_missing_time_system_xml",
            input: TDM_MISSING_TIME_SYSTEM_XML,
            should_parse: false,
            is_xml: true,
        },
        MinimalCase {
            name: "tdm_invalid_number_kvn",
            input: TDM_INVALID_NUMBER_KVN,
            should_parse: false,
            is_xml: false,
        },
        MinimalCase {
            name: "ndm_wrong_format",
            input: "NOT_A_CCSDS_MESSAGE",
            should_parse: false,
            is_xml: false,
        },
    ]
}

const ACM_MINIMAL_KVN: &str = r#"CCSDS_ACM_VERS = 2.0
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
"#;

const AEM_MINIMAL_KVN: &str = r#"CCSDS_AEM_VERS = 1.0
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
"#;

const APM_MINIMAL_KVN: &str = r#"CCSDS_APM_VERS = 2.0
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
"#;

const CDM_MINIMAL_KVN: &str = r#"CCSDS_CDM_VERS = 1.0
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

const OCM_MINIMAL_KVN: &str = r#"CCSDS_OCM_VERS = 3.0
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
"#;

const OEM_MINIMAL_KVN: &str = r#"CCSDS_OEM_VERS = 2.0
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
"#;

const OMM_MINIMAL_KVN: &str = r#"CCSDS_OMM_VERS = 2.0
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
"#;

const OPM_MINIMAL_KVN: &str = r#"CCSDS_OPM_VERS = 2.0
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
"#;

const TDM_MINIMAL_KVN: &str = r#"CCSDS_TDM_VERS = 2.0
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
"#;

const RDM_MINIMAL_KVN: &str = r#"CCSDS_RDM_VERS = 1.0
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

const NDM_MINIMAL_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
"#;

const OEM_MINIMAL_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
"#;

const TDM_MISSING_TIME_SYSTEM_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
"#;

const TDM_INVALID_NUMBER_KVN: &str = r#"CCSDS_TDM_VERS = 2.0
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
RANGE = 2023-01-01T00:00:00 BAD
DATA_STOP
"#;
