use std::fs;
use std::path::{Path, PathBuf};

use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::Ndm;

mod common;
use common::{assert_rejects, validate_xml};

const KVN: &str = include_str!("../data/kvn/apm_g1.kvn");
const XML: &str = include_str!("../data/xml/apm_g10.xml");
const SPIN_KVN: &str = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 2023-001A
TIME_SYSTEM = UTC
META_STOP
EPOCH = 2023-01-01T00:00:00
SPIN_START
REF_FRAME_A = J2000
REF_FRAME_B = SC_BODY_1
SPIN_ALPHA = 10
SPIN_DELTA = 20
SPIN_ANGLE = 30
SPIN_ANGLE_VEL = 0.1
NUTATION = 5
NUTATION_PER = 100
NUTATION_PHASE = 45
SPIN_STOP
"#;

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn apm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = TRMM";
    let object_id = "OBJECT_ID = 1997-074A";
    let q1 = "Q1 = 0.00005";
    for (label, source) in [
        (
            "duplicate top-level keyword",
            KVN.replace(object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered metadata keywords",
            KVN.replace(
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "duplicate logical-block keyword",
            KVN.replace(q1, &format!("{q1}\n{q1}")),
        ),
        (
            "unknown logical-block keyword",
            KVN.replace(q1, &format!("{q1}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            KVN.replace(q1, &format!("{q1}\nCOMMENT misplaced")),
        ),
        ("unknown block", KVN.replace("QUAT_START", "UNKNOWN_START")),
        (
            "mismatched block end",
            KVN.replace("QUAT_STOP", "SPIN_STOP"),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(object_name, &format!("{object_name} €")),
        ),
    ] {
        assert!(Apm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn apm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let epoch = "<EPOCH>2003-09-30T14:28:15.1172</EPOCH>";
    let q1 = "<Q1>0.00005</Q1>";
    for (label, source) in [
        (
            "unknown quaternion-state child",
            XML.replace("<quaternionState>", "<quaternionState><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown quaternion child",
            XML.replace("<quaternion>", "<quaternion><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown container attribute",
            XML.replace(
                "<quaternionState>",
                "<quaternionState unexpected=\"value\">",
            ),
        ),
        (
            "unknown leaf attribute",
            XML.replace(q1, "<Q1 unexpected=\"value\">0.00005</Q1>"),
        ),
        (
            "duplicate epoch",
            XML.replace(epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered quaternion components",
            XML.replace(
                "<Q1>0.00005</Q1>\n<Q2>0.87543</Q2>",
                "<Q2>0.87543</Q2>\n<Q1>0.00005</Q1>",
            ),
        ),
    ] {
        assert!(Apm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_apm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    for name in ["apm_g1.kvn", "apm_g2.kvn", "apm_g3.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Apm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Apm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Apm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    let message = Apm::from_xml(XML).unwrap();
    let xml = message.to_xml().unwrap();
    assert_eq!(Apm::from_xml(&xml).unwrap(), message);
    validate_xml("apm_g10.xml", &xml);
}

#[test]
fn apm_spin_revalidates_edited_numeric_values() {
    let base = Apm::from_kvn(SPIN_KVN).unwrap();
    let mut angle = base.clone();
    angle.body.segment.data.spin[0].spin_alpha.value = 360.0;
    assert_rejects(&angle, "SPIN_ALPHA");

    let mut rate = base.clone();
    rate.body.segment.data.spin[0].spin_angle_vel.value = f64::NAN;
    assert_rejects(&rate, "SPIN_ANGLE_VEL");

    let mut nutation = base.clone();
    nutation.body.segment.data.spin[0]
        .nutation
        .as_mut()
        .unwrap()
        .value = -360.1;
    assert_rejects(&nutation, "NUTATION");

    let mut period = base;
    period.body.segment.data.spin[0]
        .nutation_per
        .as_mut()
        .unwrap()
        .value = -1.0;
    assert_rejects(&period, "NUTATION_PER");

    let mut momentum = Apm::from_kvn(SPIN_KVN).unwrap();
    let spin = &mut momentum.body.segment.data.spin[0];
    spin.nutation = None;
    spin.nutation_per = None;
    spin.nutation_phase = None;
    spin.momentum_alpha = Some(spin.spin_alpha.clone());
    spin.momentum_delta = Some(spin.spin_delta.clone());
    spin.nutation_vel = Some(spin.spin_angle_vel.clone());
    spin.momentum_delta.as_mut().unwrap().value = 360.0;
    assert_rejects(&momentum, "MOMENTUM_DELTA");
}

/// `EulerAngleState`, `AngVelState`, and `InertiaState` had no validator reached from any root.
/// An Euler angle of 400 degrees passed `validate`, KVN, and XML, and the emitted document was
/// rejected by the reference schema on `[facet 'maxExclusive']`. Found by loosening the Python
/// setter that had been the only guard.
#[test]
fn apm_repeated_attitude_blocks_are_reached_from_the_root() {
    const EULER_KVN: &str = include_str!("../data/kvn/apm_g2.kvn");
    const INERTIA_KVN: &str = include_str!("../data/kvn/apm_g3.kvn");

    // One value is enough to prove the route is reached; `angleType`'s domain is exercised once
    // in `types.rs::test_angle_validation` rather than re-proved at every call site.
    let mut message = Apm::from_kvn(EULER_KVN).unwrap();
    message.body.segment.data.euler_angle_state[0].angle_1.value = 400.0;
    assert_rejects(&message, "EULER_ANGLE_1");

    let mut message = Apm::from_kvn(INERTIA_KVN).unwrap();
    message.body.segment.data.inertia[0].iyz.value = f64::INFINITY;
    assert_rejects(&message, "IYZ");

    // The accepted boundary still reaches the reference schema.
    let mut message = Apm::from_kvn(EULER_KVN).unwrap();
    message.body.segment.data.euler_angle_state[0].angle_1.value = -360.0;
    validate_xml("APM Euler boundary", &message.to_xml().unwrap());
}
