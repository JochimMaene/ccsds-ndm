use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::traits::Ndm;
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/omm_g9.kvn");
const XML: &str = include_str!("../data/xml/omm_g10.xml");
/// The only shipped OMM fixture that carries a covariance matrix.
const KVN_WITH_COVARIANCE: &str = include_str!("../data/kvn/omm_g8.kvn");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn assert_kvn_rejected(label: &str, source: String) {
    assert!(Omm::from_kvn(&source).is_err(), "accepted {label}");
}

#[test]
fn omm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = GOES 9";
    let object_id = "OBJECT_ID = 1995-025A";
    for (label, source) in [
        (
            "duplicate keyword",
            KVN.replace(object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered keywords",
            KVN.replace(
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            KVN.replace(object_name, &format!("{object_name}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            KVN.replace(object_name, &format!("{object_name}\nCOMMENT misplaced")),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(object_name, &format!("{object_name} €")),
        ),
    ] {
        assert_kvn_rejected(label, source);
    }
}

#[test]
fn omm_xml_rejects_unknown_nested_content_and_ordering_errors() {
    let epoch = "<EPOCH>2020-064T10:34:41.4264</EPOCH>";
    let mean_motion = "<MEAN_MOTION>1.00273272</MEAN_MOTION>";
    for (label, source) in [
        (
            "unknown mean-elements child",
            XML.replace("<meanElements>", "<meanElements><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown TLE child",
            XML.replace("<tleParameters>", "<tleParameters><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown mean-elements attribute",
            XML.replace("<meanElements>", "<meanElements unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            XML.replace(
                epoch,
                "<EPOCH unexpected=\"value\">2020-064T10:34:41.4264</EPOCH>",
            ),
        ),
        (
            "duplicate element",
            XML.replace(epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered elements",
            XML.replace(
                &format!("{epoch}\n{mean_motion}"),
                &format!("{mean_motion}\n{epoch}"),
            ),
        ),
    ] {
        assert!(Omm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_omm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    for name in ["omm_g7.kvn", "omm_g8.kvn", "omm_g9.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Omm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Omm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Omm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    let message = Omm::from_xml(XML).unwrap();
    let xml = message.to_xml().unwrap();
    assert_eq!(Omm::from_xml(&xml).unwrap(), message);
    validate_xml("omm_g10.xml", &xml);
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

/// Each OMM keyword choice shares one ordering rank so either spelling may fill the slot. That
/// allowance must not extend to repeating one alternative: the KVN block parser keeps the last
/// assignment, so a repeat that reached it would silently discard a value.
#[test]
fn omm_kvn_separates_keyword_choices_from_repeated_alternatives() {
    for (label, key, line) in [
        (
            "MEAN_MOTION",
            "MEAN_MOTION = 1.00273272 [rev/day]",
            "MEAN_MOTION = 2.0",
        ),
        ("BSTAR", "BSTAR = 0.0001 [1/ER]", "BSTAR = 0.0002"),
        (
            "MEAN_MOTION_DDOT",
            "MEAN_MOTION_DDOT = 0.0 [rev/day**3]",
            "MEAN_MOTION_DDOT = 1.0",
        ),
    ] {
        assert!(KVN.contains(key), "fixture should contain {label}");
        assert_kvn_rejected(
            &format!("repeated {label}"),
            KVN.replace(key, &format!("{key}\n{line}")),
        );
    }

    // The other alternative may still follow; it is rejected as a semantic conflict rather than
    // as an ordering error, so the diagnostic names both fields.
    let error = Omm::from_kvn(&KVN.replace(
        "BSTAR = 0.0001 [1/ER]",
        "BSTAR = 0.0001 [1/ER]\nBTERM = 0.02",
    ))
    .expect_err("BSTAR and BTERM are mutually exclusive");
    // `ValidationError::Conflict` has no stabilized code yet, so match the diagnostic itself.
    assert!(
        error.to_string().contains("Conflicting fields"),
        "expected a conflict diagnostic, got {error}"
    );
}

/// Every OMM value the schema types as a double must be a real number before generation runs.
///
/// The schema range facets are comparisons, and comparisons against NaN are false, so a range
/// check alone lets NaN through to the output document.
#[test]
fn omm_generation_rejects_non_finite_values_in_every_numeric_block() {
    /// A named mutation that puts a non-finite value into one numeric block.
    type NonFiniteCase = (&'static str, fn(&mut Omm));

    let cases: [NonFiniteCase; 6] = [
        ("mean elements eccentricity", |omm| {
            omm.body.segment.data.mean_elements.eccentricity.value = f64::NAN
        }),
        ("mean elements mean motion", |omm| {
            omm.body
                .segment
                .data
                .mean_elements
                .mean_motion
                .as_mut()
                .expect("fixture uses MEAN_MOTION")
                .value = f64::NAN
        }),
        ("mean elements GM", |omm| {
            omm.body
                .segment
                .data
                .mean_elements
                .gm
                .as_mut()
                .expect("fixture has GM")
                .value = f64::NAN
        }),
        ("TLE BSTAR", |omm| {
            omm.body
                .segment
                .data
                .tle_parameters
                .as_mut()
                .expect("fixture has TLE parameters")
                .bstar
                .as_mut()
                .expect("fixture has BSTAR")
                .value = f64::NAN
        }),
        ("TLE MEAN_MOTION_DOT", |omm| {
            omm.body
                .segment
                .data
                .tle_parameters
                .as_mut()
                .expect("fixture has TLE parameters")
                .mean_motion_dot
                .value = f64::NAN
        }),
        ("covariance CX_X", |omm| {
            omm.body
                .segment
                .data
                .covariance_matrix
                .as_mut()
                .expect("fixture has a covariance matrix")
                .cx_x
                .value = f64::NAN
        }),
    ];

    for (label, mutate) in cases {
        let mut omm = Omm::from_kvn(KVN_WITH_COVARIANCE).expect("fixture should parse");
        mutate(&mut omm);
        assert!(omm.to_kvn().is_err(), "{label} generated KVN");
        assert!(omm.to_xml().is_err(), "{label} generated XML");
    }
}

/// `inclinationType` narrows `angleRange` to `[0, 180]`, which the typed wrapper only enforces
/// through its constructor. Validation has to restate it for models that reach the field directly.
#[test]
fn omm_validation_enforces_the_inclination_range() {
    let mut omm = Omm::from_kvn(KVN).expect("fixture should parse");
    omm.body.segment.data.mean_elements.inclination.angle.value = 190.0;

    let error = omm.to_xml().expect_err("190 degrees is outside [0, 180]");
    assert_eq!(error.code(), Some("validation.out_of_range"));
}

/// The shared ODM covariance and state-vector writers must spell numbers the way ODM 7.7.1
/// requires, so that generated KVN reparses.
#[test]
fn omm_kvn_generation_spells_numbers_as_ccsds_numbers() {
    let mut omm = Omm::from_kvn(KVN_WITH_COVARIANCE).expect("fixture should parse");
    {
        let covariance = omm
            .body
            .segment
            .data
            .covariance_matrix
            .as_mut()
            .expect("fixture has a covariance matrix");
        covariance.cx_x.value = 1e-9;
        covariance.cy_x.value = 0.1 + 0.2;
        covariance.cy_y.value = 1.234_567_890_123_456_7;
    }

    let kvn = omm.to_kvn().expect("finite values should generate");
    assert!(kvn.contains("CX_X                 = 1.0e-9\n"), "{kvn}");
    assert!(kvn.contains("CY_X                 = 3.0e-1\n"), "{kvn}");
    assert!(
        kvn.contains("CY_Y                 = 1.234567890123457e0\n"),
        "{kvn}"
    );
    Omm::from_kvn(&kvn).expect("generated KVN should reparse");
}
