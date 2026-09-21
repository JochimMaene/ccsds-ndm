use crate::common::{assert_rejects, mutated, validate_xml};
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::types::ElementSetNo;
use ccsds_ndm::Ndm;

const KVN: &str = include_str!("../../data/kvn/omm_g9.kvn");
const XML: &str = include_str!("../../data/xml/omm_g10.xml");
/// The only shipped OMM fixture that carries a covariance matrix.
const KVN_WITH_COVARIANCE: &str = include_str!("../../data/kvn/omm_g8.kvn");

fn assert_kvn_rejected(label: &str, source: String) {
    let error = Omm::from_kvn(&source).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
}

#[test]
fn omm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = GOES 9";
    let object_id = "OBJECT_ID = 1995-025A";
    for (label, source) in [
        (
            "duplicate keyword",
            mutated(KVN, object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered keywords",
            mutated(
                KVN,
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            mutated(KVN, object_name, &format!("{object_name}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            mutated(
                KVN,
                object_name,
                &format!("{object_name}\nCOMMENT misplaced"),
            ),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            mutated(KVN, object_name, &format!("{object_name} €")),
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
            mutated(XML, "<meanElements>", "<meanElements><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown TLE child",
            mutated(
                XML,
                "<tleParameters>",
                "<tleParameters><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown mean-elements attribute",
            mutated(XML, "<meanElements>", "<meanElements unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            mutated(
                XML,
                epoch,
                "<EPOCH unexpected=\"value\">2020-064T10:34:41.4264</EPOCH>",
            ),
        ),
        (
            "duplicate element",
            mutated(XML, epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered elements",
            mutated(
                XML,
                &format!("{epoch}\n{mean_motion}"),
                &format!("{mean_motion}\n{epoch}"),
            ),
        ),
    ] {
        let error = Omm::from_xml(&source).unwrap_err();
        assert!(
            matches!(
                error.as_format_error(),
                Some(ccsds_ndm::error::FormatError::InvalidFormat(_))
            ),
            "{label}: {error}"
        );
    }
}

/// `elementSetNoType` restricts to `[0, 9999]` in both bundled OMM schemas, but `ElementSetNo`
/// only enforces that in its constructor and the value field is public. Before this was routed
/// through the root, `ELEMENT_SET_NO = 100000` produced XML that `xmllint` rejected with
/// `[facet 'maxInclusive']`.
#[test]
fn omm_validation_enforces_the_element_set_number_range() {
    let mut omm = Omm::from_kvn(KVN_WITH_COVARIANCE).expect("fixture should parse");
    omm.body
        .segment
        .data
        .tle_parameters
        .as_mut()
        .expect("fixture has TLE parameters")
        .element_set_no = Some(ElementSetNo { value: 100_000 });

    assert_rejects(&omm, "ELEMENT_SET_NO");

    // The accepted boundary must still reach the reference schema.
    omm.body
        .segment
        .data
        .tle_parameters
        .as_mut()
        .unwrap()
        .element_set_no = Some(ElementSetNo { value: 9999 });
    omm.validate().expect("9999 is the inclusive maximum");
    validate_xml("OMM ELEMENT_SET_NO boundary", &omm.to_xml().unwrap());
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
            mutated(KVN, key, &format!("{key}\n{line}")),
        );
    }

    // The other alternative may still follow; it is rejected as a semantic conflict rather than
    // as an ordering error, so the diagnostic names both fields.
    let error = Omm::from_kvn(&mutated(
        KVN,
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

#[test]
fn omm_rejects_kvn_numbers_it_could_not_spell_back() {
    // The schema types these fields as plain doubles, so finiteness and range checks let through
    // magnitudes whose shortest round-tripping spelling blows past the 254-character line limit.
    let mut message = Omm::from_kvn(include_str!("../../data/kvn/omm_g7.kvn")).unwrap();
    message.body.segment.data.mean_elements.eccentricity.value = f64::MAX;

    let error = message.to_kvn().unwrap_err().to_string();
    assert!(
        error.contains("representable CCSDS number"),
        "unexpected error: {error}"
    );
}

#[test]
fn omm_kvn_numbers_use_odm_spellings_in_every_block() {
    let mut message = Omm::from_kvn(include_str!("../../data/kvn/omm_g7.kvn")).unwrap();
    let data = &mut message.body.segment.data;
    data.mean_elements.eccentricity.value = 0.1 + 0.2;
    data.mean_elements.mean_motion.as_mut().unwrap().value = 0.1 + 0.2;
    let tle = data.tle_parameters.as_mut().unwrap();
    tle.mean_motion_dot.value = 0.1 + 0.2;

    let kvn = message.to_kvn().unwrap();
    assert!(
        !kvn.contains("0.30000000000000004"),
        "Display spelling leaked into generated KVN:\n{kvn}"
    );
    for key in ["ECCENTRICITY", "MEAN_MOTION ", "MEAN_MOTION_DOT"] {
        let line = kvn
            .lines()
            .find(|line| line.starts_with(key))
            .unwrap_or_else(|| panic!("missing {key}"));
        assert!(
            !line.contains("0000000000000"),
            "unexpected {key} line: {line}"
        );
        assert!(line.len() <= 254, "{key} line exceeds 254 characters");
    }
}

#[test]
fn omm_xml_allows_a_comment_to_reopen_the_user_defined_group() {
    // The OMM carries the same `userDefinedType` block as the OPM and the RDM, so it has to
    // accept the repeating group on the same terms.
    let source = mutated(
        include_str!("../../data/xml/omm_g10.xml"),
        "</data>",
        "<userDefinedParameters>\
         <COMMENT>first</COMMENT>\
         <USER_DEFINED parameter=\"A\">1</USER_DEFINED>\
         <COMMENT>second</COMMENT>\
         <USER_DEFINED parameter=\"B\">2</USER_DEFINED>\
         </userDefinedParameters></data>",
    );
    let message = Omm::from_xml(&source).unwrap();
    let user_defined = message
        .body
        .segment
        .data
        .user_defined_parameters
        .as_ref()
        .unwrap();
    assert_eq!(user_defined.comment, ["first", "second"]);
    assert_eq!(user_defined.user_defined.len(), 2);
}

#[test]
fn omm_xml_still_rejects_a_genuinely_out_of_order_child() {
    // The relaxation is scoped to the repeating group: `metadata` is a plain xsd:sequence, so
    // CENTER_NAME cannot precede OBJECT_ID.
    let source = include_str!("../../data/xml/omm_g10.xml");
    let reordered = mutated(
        source,
        "<OBJECT_ID>1995-025A</OBJECT_ID>\n<CENTER_NAME>EARTH</CENTER_NAME>",
        "<CENTER_NAME>EARTH</CENTER_NAME>\n<OBJECT_ID>1995-025A</OBJECT_ID>",
    );
    assert_ne!(source, reordered, "fixture shape changed; update this test");
    crate::common::assert_invalid_format(
        &Omm::from_xml(&reordered).unwrap_err(),
        "invalid OMM XML sequence: duplicate or out-of-order child 'OBJECT_ID' in 'metadata'",
    );
}
