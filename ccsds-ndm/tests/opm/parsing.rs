// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{assert_validation_field, mutated, mutated_once};
use crate::{opm_with_maneuvers, MINIMAL};
use ccsds_ndm::error::{DiagnosticNotation, DiagnosticOperation, ValidationError};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::types::{CalendarEpoch, NonNegativeDouble};
use ccsds_ndm::validation::MessageKind;
use ccsds_ndm::Ndm;

const KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");

fn assert_kvn_rejected(label: &str, kvn: String) {
    let error = Opm::from_kvn(&kvn).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
}

#[test]
fn strict_kvn_rejects_unknown_duplicate_reordered_malformed_and_lossy_content() {
    let object_name = KVN
        .lines()
        .find(|line| line.starts_with("OBJECT_NAME"))
        .expect("fixture should contain OBJECT_NAME");
    assert_kvn_rejected(
        "duplicate fixed keyword",
        mutated(KVN, object_name, &format!("{object_name}\n{object_name}")),
    );

    let object_id = KVN
        .lines()
        .find(|line| line.starts_with("OBJECT_ID"))
        .expect("fixture should contain OBJECT_ID");
    assert_kvn_rejected(
        "reordered fixed keywords",
        mutated(
            KVN,
            &format!("{object_name}\n{object_id}"),
            &format!("{object_id}\n{object_name}"),
        ),
    );
    assert_kvn_rejected(
        "unknown keyword",
        mutated(
            KVN,
            object_name,
            &format!("{object_name}\nUNKNOWN_KEY = value"),
        ),
    );
    assert_kvn_rejected(
        "comment in the middle of a logical block",
        mutated(
            KVN,
            object_name,
            &format!("{object_name}\nCOMMENT misplaced"),
        ),
    );
    assert_kvn_rejected(
        "leading comment that cannot be represented",
        format!("COMMENT lost\n{KVN}"),
    );
    assert_kvn_rejected(
        "non-printable character",
        mutated(KVN, object_name, &format!("{object_name}\u{1}")),
    );

    let version = KVN.lines().next().expect("fixture should have a version");
    assert_kvn_rejected(
        "line longer than 254 characters",
        mutated(
            KVN,
            version,
            &format!("{version}\nCOMMENT {}", "x".repeat(250)),
        ),
    );
    assert_kvn_rejected(
        "lone carriage return",
        mutated(KVN, "OBJECT_NAME", "OBJECT\r_NAME"),
    );
}

#[test]
fn strict_kvn_accepts_every_normative_line_ending_without_changing_meaning() {
    let lf = Opm::from_kvn(KVN).expect("LF fixture should parse");
    for (name, ending) in [("CR", "\r"), ("CRLF", "\r\n"), ("LFCR", "\n\r")] {
        let source = mutated(KVN, "\n", ending);
        let parsed = Opm::from_kvn(&source)
            .unwrap_or_else(|error| panic!("{name} fixture should parse: {error}"));
        assert_eq!(parsed, lf, "{name} changed the parsed model");
    }
}

/// `TRUE_ANOMALY` and `MEAN_ANOMALY` share an ordering rank so either may fill the single
/// anomaly slot. That allowance must not extend to repeating one of them: `parse_block!` keeps
/// the last assignment, so a repeat that reached the parser would silently discard a value.
#[test]
fn strict_kvn_separates_the_anomaly_choice_from_a_repeated_anomaly() {
    let keplerian = "SEMI_MAJOR_AXIS = 6800.0
ECCENTRICITY = 0.0005
INCLINATION = 51.6
RA_OF_ASC_NODE = 10.0
ARG_OF_PERICENTER = 20.0
";
    let with_anomalies = |anomalies: &str| {
        let mut kvn = KVN
            .lines()
            .take_while(|line| !line.starts_with("MASS"))
            .collect::<Vec<_>>()
            .join("\n");
        kvn.push('\n');
        kvn.push_str(keplerian);
        kvn.push_str(anomalies);
        kvn.push_str("GM = 398600.4418\n");
        kvn
    };

    Opm::from_kvn(&with_anomalies("TRUE_ANOMALY = 30.0\n"))
        .expect("a single TRUE_ANOMALY fills the choice");
    Opm::from_kvn(&with_anomalies("MEAN_ANOMALY = 30.0\n"))
        .expect("a single MEAN_ANOMALY fills the choice");

    assert_kvn_rejected(
        "repeated TRUE_ANOMALY",
        with_anomalies("TRUE_ANOMALY = 30.0\nTRUE_ANOMALY = 40.0\n"),
    );
    assert_kvn_rejected(
        "repeated MEAN_ANOMALY",
        with_anomalies("MEAN_ANOMALY = 30.0\nMEAN_ANOMALY = 40.0\n"),
    );

    // Both alternatives present remains a semantic choice violation, not an ordering error, so
    // the diagnostic still names the pair.
    for order in [
        "TRUE_ANOMALY = 30.0\nMEAN_ANOMALY = 40.0\n",
        "MEAN_ANOMALY = 30.0\nTRUE_ANOMALY = 40.0\n",
    ] {
        let error =
            Opm::from_kvn(&with_anomalies(order)).expect_err("only one anomaly may be present");
        assert_eq!(error.code(), Some("validation.invalid_choice"));
    }
}

fn xml() -> String {
    Opm::from_kvn(include_str!("../../data/kvn/opm_g1.kvn"))
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML")
}

fn assert_xml_rejected(label: &str, xml: String) {
    let error = Opm::from_xml(&xml).unwrap_err();
    assert_eq!(error.code(), Some("parse.xml.syntax"), "{label}: {error}");
}

#[test]
fn strict_xml_rejects_wrong_root_unknown_content_duplicates_and_trailing_documents() {
    let source = xml();
    assert_xml_rejected(
        "wrong root",
        mutated(&mutated(&source, "<opm ", "<omm "), "</opm>", "</omm>"),
    );
    assert_xml_rejected(
        "unknown root attribute",
        mutated(&source, "<opm ", "<opm unexpected=\"value\" "),
    );
    assert_xml_rejected(
        "unknown metadata element",
        mutated(&source, "<metadata>", "<metadata><UNKNOWN>value</UNKNOWN>"),
    );
    assert_xml_rejected(
        "unknown metadata attribute",
        mutated(&source, "<metadata>", "<metadata unexpected=\"value\">"),
    );

    let object_name = "<OBJECT_NAME>OSPREY 5</OBJECT_NAME>";
    assert!(source.contains(object_name));
    assert_xml_rejected(
        "duplicate fixed element",
        mutated(&source, object_name, &format!("{object_name}{object_name}")),
    );
    let object_id = "<OBJECT_ID>1998-999A</OBJECT_ID>";
    assert!(source.contains(&format!("{object_name}{object_id}")));
    assert_xml_rejected(
        "reordered metadata elements",
        mutated(
            &source,
            &format!("{object_name}{object_id}"),
            &format!("{object_id}{object_name}"),
        ),
    );
    assert_xml_rejected("trailing element", format!("{source}<junk/>"));
    assert_xml_rejected("multiple documents", format!("{source}{source}"));
    assert_xml_rejected(
        "document type declaration",
        mutated_once(&source, "<opm ", "<!DOCTYPE opm><opm "),
    );
}

#[test]
fn strict_xml_rejects_attributes_the_schema_does_not_declare() {
    let source = Opm::from_kvn(include_str!("../../data/kvn/opm_g2.kvn"))
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML");

    for (label, from, to) in [
        (
            "unknown attribute on a measure element",
            "<X units=\"km\">",
            "<X units=\"km\" unexpected=\"value\">",
        ),
        (
            "unknown attribute on a text element",
            "<OBJECT_NAME>",
            "<OBJECT_NAME unexpected=\"value\">",
        ),
        (
            "units on a unitless element",
            "<ECCENTRICITY>",
            "<ECCENTRICITY units=\"km\">",
        ),
        (
            "units on an epoch element",
            "<EPOCH>",
            "<EPOCH units=\"s\">",
        ),
        (
            "nil on a mandatory element",
            "<X units=\"km\">",
            "<X units=\"km\" nil=\"true\">",
        ),
        (
            "parameter attribute outside USER_DEFINED",
            "<MASS units=\"kg\">",
            "<MASS units=\"kg\" parameter=\"FOO\">",
        ),
    ] {
        assert!(source.contains(from), "fixture should contain {from}");
        assert_xml_rejected(label, mutated(&source, from, to));
    }
    // Nil cannot remove the required mass from a present spacecraft block.
    crate::common::assert_validation_field(
        &Opm::from_xml(&mutated(
            &source,
            "<MASS units=\"kg\">",
            "<MASS units=\"km\" nil=\"true\">",
        ))
        .unwrap_err(),
        "MASS",
    );
}

#[test]
fn strict_xml_keeps_schema_attributes_and_the_documented_nil_extension() {
    let source = Opm::from_kvn(include_str!("../../data/kvn/opm_g2.kvn"))
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML");

    // `units` is schema-defined; `nil` on an otherwise attribute-free optional value is a
    // documented compatibility extension.
    Opm::from_xml(&source).expect("generated XML should round-trip");
    let with_nil = mutated(
        &source,
        "<DRAG_COEFF>2.3</DRAG_COEFF>",
        "<DRAG_COEFF nil=\"true\"/>",
    );
    assert_ne!(with_nil, source, "fixture should contain DRAG_COEFF");
    let parsed = Opm::from_xml(&with_nil).expect("nil-marked optional value should be accepted");
    assert!(
        parsed
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .is_some_and(|parameters| parameters.drag_coeff.is_none()),
        "nil DRAG_COEFF should deserialize as absent"
    );
}

#[test]
fn strict_xml_accepts_namespace_metadata_and_does_not_confuse_processing_instructions() {
    let source = xml();
    let source = mutated_once(
        &source,
        "?>",
        "?><?xml-stylesheet type=\"text/xsl\" href=\"opm.xsl\"?>",
    );
    let source = mutated_once(
        &source,
        "<opm ",
        "<opm xmlns:ndm=\"urn:ccsds:ndm\" xsi:schemaLocation=\"urn:ccsds:ndm opm.xsd\" ",
    );
    Opm::from_xml(&source).expect("namespace metadata and xml-stylesheet PI should parse");

    assert_xml_rejected(
        "default namespace that qualifies the OPM root",
        mutated_once(&xml(), "<opm ", "<opm xmlns=\"urn:ccsds:ndm\" "),
    );
}

#[test]
fn kvn_parse_diagnostic_is_located_bounded_and_machine_readable() {
    let input = mutated(
        KVN,
        "OBJECT_NAME = OSPREY 5",
        "OBJECT_NAME = OSPREY 5\nUNKNOWN_KEY = value",
    );
    let error = Opm::from_kvn(&input).expect_err("unknown keyword should fail");
    let diagnostic = error
        .diagnostic()
        .expect("strict parser should attach diagnostic context");

    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Kvn);
    assert_eq!(diagnostic.message_kind, MessageKind::Opm);
    assert_eq!(diagnostic.source_edition, Some("3.0"));
    assert_eq!(diagnostic.code, Some("parse.kvn.syntax"));
    assert_eq!(diagnostic.source_location, Some((6, 1)));
    assert_eq!(diagnostic.original_token, Some("UNKNOWN_KEY = value"));
    assert_eq!(diagnostic.expected, Some("strict OPM KVN"));
}

#[test]
fn parse_token_excerpt_is_bounded_and_semantic_paths_survive_the_context_wrapper() {
    let long = mutated(
        KVN,
        "OBJECT_NAME = OSPREY 5",
        &format!("OBJECT_NAME = {}", "x".repeat(300)),
    );
    let error = Opm::from_kvn(&long).expect_err("overlong line should fail");
    assert!(
        error
            .diagnostic()
            .and_then(|diagnostic| diagnostic.original_token)
            .expect("long token should have an excerpt")
            .len()
            <= 128
    );

    let empty_name = mutated(KVN, "OBJECT_NAME = OSPREY 5", "OBJECT_NAME =");
    let error = Opm::from_kvn(&empty_name).expect_err("empty required field should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");
    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.code, Some("validation.missing_required_field"));
    assert_eq!(
        diagnostic.field_path.as_deref(),
        Some("body.segment.metadata.object_name")
    );
}

#[test]
fn xml_parse_diagnostic_identifies_input_notation_without_inventing_a_location() {
    let xml = Opm::from_kvn(KVN)
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML");
    let wrong_root = mutated(&mutated(&xml, "<opm ", "<omm "), "</opm>", "</omm>");
    let error = Opm::from_xml(&wrong_root).expect_err("wrong root should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");

    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Xml);
    assert_eq!(diagnostic.message_kind, MessageKind::Opm);
    assert_eq!(diagnostic.code, Some("parse.xml.syntax"));
    assert_eq!(diagnostic.source_location, None);
    assert_eq!(diagnostic.original_token, None);
}

#[test]
fn xml_source_edition_comes_from_the_parsed_root() {
    let xml = mutated(
        &mutated_once(
            &mutated_once(
                &Opm::from_kvn(KVN)
                    .expect("fixture should parse")
                    .to_xml()
                    .expect("fixture should generate XML"),
                "?>",
                "?><!-- <opm version=\"bogus\"> -->",
            ),
            "version=\"3.0\"",
            "version='3.0'",
        ),
        "<OBJECT_NAME>OSPREY 5</OBJECT_NAME>",
        "<OBJECT_NAME></OBJECT_NAME>",
    );

    let error = Opm::from_xml(&xml).expect_err("empty object name should fail validation");
    assert_eq!(
        error
            .diagnostic()
            .expect("parse context should be present")
            .source_edition,
        Some("3.0")
    );
}

#[test]
fn calendar_epoch_rejects_invalid_opm_values() {
    for value in [
        "",
        "+",
        ".",
        "12345",
        "12345.",
        "2023-02-29T00:00:00",
        "2023-01-01T24:00:00",
        "2023-01-01T00:60:00",
        "2023-01-01T00:00:00.",
    ] {
        assert!(
            matches!(CalendarEpoch::new(value).unwrap_err(), ccsds_ndm::types::EpochError::InvalidFormat(actual) if actual == value)
        );
    }
}

fn replace_element(xml: &str, tag: &str, value: &str) -> String {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml
        .find(&open)
        .unwrap_or_else(|| panic!("missing opening tag {open}"));
    let content_start = start + open.len();
    let end = xml[content_start..]
        .find(&close)
        .map(|offset| content_start + offset)
        .unwrap_or_else(|| panic!("missing closing tag {close}"));
    format!(
        "{}{}{}{}{}",
        &xml[..start],
        open,
        value,
        close,
        &xml[end + close.len()..]
    )
}

#[test]
fn xml_parsing_rejects_non_calendar_opm_epochs() {
    let xml = mutated_once(
        &opm_with_maneuvers()
            .to_xml()
            .expect("fixture should serialize"),
        "</REF_FRAME>",
        "</REF_FRAME><REF_FRAME_EPOCH>2023-01-01T00:00:00</REF_FRAME_EPOCH>",
    );
    for tag in ["REF_FRAME_EPOCH", "EPOCH", "MAN_EPOCH_IGNITION"] {
        let invalid = replace_element(&xml, tag, "12345.5");
        crate::common::assert_invalid_epoch(&Opm::from_xml(&invalid).unwrap_err(), "12345.5");
    }
}

#[test]
fn xml_parsing_rejects_book_forbidden_timezone_offsets() {
    let xml = opm_with_maneuvers()
        .to_xml()
        .expect("fixture should serialize");
    for tag in ["CREATION_DATE", "EPOCH", "MAN_EPOCH_IGNITION"] {
        let invalid = replace_element(&xml, tag, "2023-01-01T00:00:00+05:00");
        crate::common::assert_invalid_epoch(
            &Opm::from_xml(&invalid).unwrap_err(),
            "2023-01-01T00:00:00+05:00",
        );
    }
}

#[test]
fn xml_parsing_rejects_non_calendar_odm_creation_date() {
    let xml = opm_with_maneuvers()
        .to_xml()
        .expect("fixture should serialize");
    let invalid = mutated_once(
        &xml,
        "<CREATION_DATE>2021-06-03T05:33:00.000</CREATION_DATE>",
        "<CREATION_DATE>12345</CREATION_DATE>",
    );
    crate::common::assert_invalid_epoch(&Opm::from_xml(&invalid).unwrap_err(), "12345");
}

#[test]
fn opm_preserves_comment_before_user_defined_after_optional_maneuvers() {
    let source = mutated(
        include_str!("../../data/kvn/opm_g4.kvn"),
        "USER_DEFINED_EARTH_MODEL",
        "COMMENT belongs to user parameters\nUSER_DEFINED_EARTH_MODEL",
    );
    let message = Opm::from_kvn(&source).unwrap();
    assert_eq!(
        message
            .body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap()
            .comment,
        ["belongs to user parameters"]
    );
}

#[test]
fn opm_rejects_maneuver_fields_without_ignition_epoch() {
    let source = mutated(
        include_str!("../../data/kvn/opm_g4.kvn"),
        "USER_DEFINED_EARTH_MODEL",
        "MAN_DURATION = 10.0 [s]\n\
         MAN_DELTA_MASS = -1.0 [kg]\n\
         MAN_REF_FRAME = TNW\n\
         MAN_DV_1 = 0.0 [km/s]\n\
         MAN_DV_2 = 0.0 [km/s]\n\
         MAN_DV_3 = 0.0 [km/s]\n\
         USER_DEFINED_EARTH_MODEL",
    );
    let error = Opm::from_kvn(&source).expect_err("maneuver without its ignition epoch accepted");
    assert!(
        error
            .to_string()
            .contains("Missing required field: MAN_EPOCH_IGNITION"),
        "unexpected diagnostic: {error}"
    );
}

#[test]
fn opm_xml_allows_a_comment_to_reopen_the_user_defined_group() {
    // `userDefinedType` wraps COMMENT and USER_DEFINED in an unbounded xsd:sequence, so the pair
    // may repeat and a COMMENT may legally follow a USER_DEFINED.
    let source = mutated(
        include_str!("../../data/xml/opm_g5.xml"),
        "</data>",
        "<userDefinedParameters>\
         <COMMENT>first</COMMENT>\
         <USER_DEFINED parameter=\"A\">1</USER_DEFINED>\
         <COMMENT>second</COMMENT>\
         <USER_DEFINED parameter=\"B\">2</USER_DEFINED>\
         </userDefinedParameters></data>",
    );
    let message = Opm::from_xml(&source).unwrap();
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
fn opm_xml_still_rejects_a_genuinely_out_of_order_child() {
    // Relaxing the rank check for repeating groups must not relax a plain sequence: `header` is
    // an ordinary xsd:sequence, so ORIGINATOR cannot precede CREATION_DATE.
    let source = mutated(
        include_str!("../../data/xml/opm_g5.xml"),
        "<CREATION_DATE>",
        "<ORIGINATOR>early</ORIGINATOR><CREATION_DATE>",
    );
    let error = Opm::from_xml(&source).unwrap_err().to_string();
    assert!(
        error.contains("out-of-order child 'CREATION_DATE'"),
        "unexpected error: {error}"
    );
}

#[test]
fn nilled_elements_may_carry_their_units_attribute() {
    // XSD lets a nillable element keep its attributes, and `attribute_allowed` explicitly permits
    // `units` on MASS, so the envelope validator and serde have to agree that this parses.
    let source = include_str!("../../data/xml/opm_g5.xml");
    let mass = source
        .lines()
        .find(|line| line.contains("<MASS"))
        .expect("fixture carries MASS");
    let nilled = mutated(source, mass.trim(), "<MASS units=\"kg\" xsi:nil=\"true\"/>");
    let message = Opm::from_xml(&nilled).unwrap();
    let parameters = message
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .expect("fixture carries a spacecraft parameters block");
    assert!(parameters.mass.is_none());
    assert!(
        parameters.drag_coeff.is_some(),
        "the rest of the block still parses"
    );
}

#[test]
fn required_metadata_and_state_fields_cannot_be_omitted() {
    Opm::from_kvn(MINIMAL).unwrap();
    for key in [
        "OBJECT_NAME",
        "OBJECT_ID",
        "CENTER_NAME",
        "REF_FRAME",
        "TIME_SYSTEM",
        "EPOCH",
        "X",
        "Y",
        "Z",
        "X_DOT",
        "Y_DOT",
        "Z_DOT",
    ] {
        let line = MINIMAL
            .lines()
            .find(|line| line.starts_with(&format!("{key} =")))
            .unwrap();
        let invalid = mutated(MINIMAL, &format!("{line}\n"), "");
        assert_validation_field(&Opm::from_kvn(&invalid).unwrap_err(), key);
    }
}

fn sample_opm_kvn() -> String {
    r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
MESSAGE_ID = OPM 201113719185
COMMENT GEOCENTRIC, CARTESIAN, EARTH FIXED
OBJECT_NAME = OSPREY 5
OBJECT_ID = 2022-999A
CENTER_NAME = EARTH
REF_FRAME = ITRF1997
TIME_SYSTEM = UTC
EPOCH = 2022-12-18T14:28:15.1172
X = 6503.514000 [km]
Y = 1239.647000 [km]
Z = -717.490000 [km]
X_DOT = -0.873160 [km/s]
Y_DOT = 8.740420 [km/s]
Z_DOT = -4.191076 [km/s]
MASS = 3000.000000 [kg]
SOLAR_RAD_AREA = 18.770000 [m**2]
SOLAR_RAD_COEFF = 1.000000
DRAG_AREA = 18.770000 [m**2]
DRAG_COEFF = 2.500000
"#
    .to_string()
}

#[test]
fn parse_opm_success() {
    let kvn = sample_opm_kvn();
    let opm = Opm::from_kvn(&kvn).expect("OPM parse failed");

    assert_eq!(opm.version, "3.0");
    assert_eq!(opm.header.originator, "JAXA");
    assert_eq!(opm.body.segment.metadata.object_name, "OSPREY 5");
    assert_eq!(opm.body.segment.data.state_vector.x.value, 6503.514);
    assert_eq!(
        opm.body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap()
            .mass
            .as_ref()
            .unwrap()
            .value,
        3000.0
    );
}

#[test]
fn parse_opm_with_maneuvers() {
    let kvn = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2000-06-03T05:33:00
ORIGINATOR = NASA
OBJECT_NAME = EUTELSAT W4
OBJECT_ID = 2000-028A
CENTER_NAME = EARTH
REF_FRAME = TOD
TIME_SYSTEM = UTC
EPOCH = 2000-06-03T00:00:00.000
X = 6655.9942 [km]
Y = -40218.5751 [km]
Z = -82.9177 [km]
X_DOT = 3.11548207 [km/s]
Y_DOT = 0.47042605 [km/s]
Z_DOT = -0.00101490 [km/s]
MASS = 1000.0 [kg]
MAN_EPOCH_IGNITION = 2000-06-03T04:23:00
MAN_DURATION = 1500.0 [s]
MAN_DELTA_MASS = -10.5 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 10.5 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
MAN_EPOCH_IGNITION = 2000-06-05T06:00:00
MAN_DURATION = 1500.0 [s]
MAN_DELTA_MASS = -10.5 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = -10.5 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#;
    let opm = Opm::from_kvn(kvn).expect("OPM maneuver parse failed");
    assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 2);
    assert_eq!(
        opm.body.segment.data.maneuver_parameters[0].man_dv_1.value,
        10.5
    );
    assert_eq!(
        opm.body.segment.data.maneuver_parameters[1].man_dv_1.value,
        -10.5
    );
}

#[test]
fn opm_maneuver_requires_mass_in_strict_mode() {
    let kvn = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2022-12-18T14:28:15.1172
X = 6503.514
Y = 1239.647
Z = -717.490
X_DOT = -0.873160
Y_DOT = 8.740420
Z_DOT = -4.191076
MAN_EPOCH_IGNITION = 2023-01-01T00:00:00
MAN_DURATION = 10.0
MAN_DELTA_MASS = -1.0
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1
MAN_DV_2 = 0.0
MAN_DV_3 = 0.0
"#;
    let err = Opm::from_kvn(kvn).unwrap_err();
    let ok = err.as_validation_error().is_some_and(|e| {
        matches!(
            e,
            ValidationError::MissingRequiredField { block, field, .. }
            if block.as_ref() == "Spacecraft Parameters" && field.as_ref() == "MASS"
        )
    });
    assert!(ok, "expected MASS missing validation error, got {err}");
}

#[test]
fn metadata_optional_ref_frame_epoch() {
    // XSD: REF_FRAME_EPOCH has minOccurs="0" - it's optional
    let kvn_without = MINIMAL;
    let opm = Opm::from_kvn(kvn_without).unwrap();
    assert!(opm.body.segment.metadata.ref_frame_epoch.is_none());

    let kvn_with = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = TEME
REF_FRAME_EPOCH = 2000-01-01T12:00:00
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 1000 [km]
Y = 2000 [km]
Z = 3000 [km]
X_DOT = 1.0 [km/s]
Y_DOT = 2.0 [km/s]
Z_DOT = 3.0 [km/s]
"#;
    let opm = Opm::from_kvn(kvn_with).unwrap();
    assert!(opm.body.segment.metadata.ref_frame_epoch.is_some());
}

#[test]
fn state_vector_all_mandatory() {
    // XSD: stateVectorType requires all position and velocity components
    let kvn = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 6503.514 [km]
Y = 1239.647 [km]
Z = -717.490 [km]
X_DOT = -0.873160 [km/s]
Y_DOT = 8.740420 [km/s]
Z_DOT = -4.191076 [km/s]
"#;
    let opm = Opm::from_kvn(kvn).unwrap();
    let sv = &opm.body.segment.data.state_vector;
    assert_eq!(sv.x.value, 6503.514);
    assert_eq!(sv.y.value, 1239.647);
    assert_eq!(sv.z.value, -717.490);
    assert_eq!(sv.x_dot.value, -0.873160);
    assert_eq!(sv.y_dot.value, 8.740420);
    assert_eq!(sv.z_dot.value, -4.191076);
}

#[test]
fn keplerian_with_true_anomaly() {
    // XSD: keplerianElementsType choice: TRUE_ANOMALY path
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = 90 [deg]
ARG_OF_PERICENTER = 180 [deg]
TRUE_ANOMALY = 270 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert!(kep.true_anomaly.is_some());
    assert!(kep.mean_anomaly.is_none());
    assert_eq!(kep.true_anomaly.as_ref().unwrap().value, 270.0);
}

#[test]
fn keplerian_with_mean_anomaly() {
    // XSD: keplerianElementsType choice: MEAN_ANOMALY path
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = 90 [deg]
ARG_OF_PERICENTER = 180 [deg]
MEAN_ANOMALY = 120 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert!(kep.mean_anomaly.is_some());
    assert!(kep.true_anomaly.is_none());
    assert_eq!(kep.mean_anomaly.as_ref().unwrap().value, 120.0);
}

#[test]
fn keplerian_eccentricity_zero_valid() {
    // XSD: nonNegativeDouble - minInclusive=0.0 (circular orbit)
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.0
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = 90 [deg]
ARG_OF_PERICENTER = 0 [deg]
TRUE_ANOMALY = 0 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.eccentricity, NonNegativeDouble::new(0.0).unwrap());
}

#[test]
fn keplerian_inclination_boundaries() {
    // XSD: inclinationType - 0 to 180 degrees inclusive
    let kvn_zero = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 0 [deg]
RA_OF_ASC_NODE = 0 [deg]
ARG_OF_PERICENTER = 0 [deg]
TRUE_ANOMALY = 0 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn_zero).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.inclination.angle.value, 0.0);

    let kvn_180 = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 180 [deg]
RA_OF_ASC_NODE = 0 [deg]
ARG_OF_PERICENTER = 0 [deg]
TRUE_ANOMALY = 0 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn_180).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.inclination.angle.value, 180.0);
}

#[test]
fn keplerian_angle_range_negative() {
    // XSD: angleRange - can be negative (minInclusive=-360.0)
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = -180 [deg]
ARG_OF_PERICENTER = -90 [deg]
TRUE_ANOMALY = -45 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.ra_of_asc_node.value, -180.0);
    assert_eq!(kep.arg_of_pericenter.value, -90.0);
    assert_eq!(kep.true_anomaly.as_ref().unwrap().value, -45.0);
}

#[test]
fn keplerian_gm_positive() {
    // XSD: positiveDouble for GM - minExclusive=0.0
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = 90 [deg]
ARG_OF_PERICENTER = 180 [deg]
TRUE_ANOMALY = 0 [deg]
GM = 0.001 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.gm.value, 0.001);
}

#[test]
fn spacecraft_parameters_with_all_fields() {
    // XSD: spacecraftParametersType has MASS, SOLAR_RAD_AREA, SOLAR_RAD_COEFF, DRAG_AREA, DRAG_COEFF
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 500 [kg]
SOLAR_RAD_AREA = 10.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 8.0 [m**2]
DRAG_COEFF = 2.2
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let sp = opm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .unwrap();
    assert_eq!(sp.mass.as_ref().unwrap().value, 500.0);
    assert_eq!(sp.solar_rad_area.as_ref().unwrap().value, 10.0);
    assert_eq!(
        sp.solar_rad_coeff.as_ref().unwrap(),
        &NonNegativeDouble::new(1.2).unwrap()
    );
    assert_eq!(sp.drag_area.as_ref().unwrap().value, 8.0);
    assert_eq!(
        sp.drag_coeff.as_ref().unwrap(),
        &NonNegativeDouble::new(2.2).unwrap()
    );
}

#[test]
fn spacecraft_zero_coefficients() {
    // XSD: nonNegativeDouble allows 0 for coefficients
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 100 [kg]
SOLAR_RAD_COEFF = 0.0
DRAG_COEFF = 0.0
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let sp = opm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .unwrap();
    assert_eq!(
        sp.solar_rad_coeff.as_ref().unwrap(),
        &NonNegativeDouble::new(0.0).unwrap()
    );
    assert_eq!(
        sp.drag_coeff.as_ref().unwrap(),
        &NonNegativeDouble::new(0.0).unwrap()
    );
}

#[test]
fn covariance_matrix_present() {
    // XSD: covarianceMatrixType when present
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"COV_REF_FRAME = RSW
CX_X = 1.0e-6 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0e-6 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0e-6 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 1.0e-9 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 1.0e-9 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 1.0e-9 [km**2/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let cov = opm.body.segment.data.covariance_matrix.as_ref().unwrap();
    assert!(cov.cov_ref_frame.is_some());
}

#[test]
fn single_maneuver() {
    // XSD: maneuverParametersType with mandatory fields
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 3000.000000 [kg]
MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = -5.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 1);
    let man = &opm.body.segment.data.maneuver_parameters[0];
    assert_eq!(man.man_duration.value, 100.0);
    assert_eq!(man.man_delta_mass.value, -5.0);
}

#[test]
fn multiple_maneuvers_unbounded() {
    // XSD: maxOccurs="unbounded" allows multiple maneuvers
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 3000.000000 [kg]
MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = -5.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
MAN_EPOCH_IGNITION = 2023-01-03T00:00:00
MAN_DURATION = 50 [s]
MAN_DELTA_MASS = -2.5 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.05 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
MAN_EPOCH_IGNITION = 2023-01-04T00:00:00
MAN_DURATION = 75 [s]
MAN_DELTA_MASS = -3.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.0 [km/s]
MAN_DV_2 = 0.1 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 3);
}

#[test]
fn maneuver_delta_mass_zero_allowed() {
    // XSD: deltamassTypeZ is nonPositiveDouble (≤0), so zero is allowed
    // This represents attitude maneuvers that don't use propellant
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 3000.000000 [kg]
MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = 0.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    // XSD allows zero for attitude maneuvers
    let opm = Opm::from_kvn(kvn).unwrap();
    let man = &opm.body.segment.data.maneuver_parameters[0];
    assert_eq!(man.man_delta_mass.value, 0.0);
}

#[test]
fn maneuver_delta_mass_positive_rejected() {
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = 5.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    // Should fail - positive MAN_DELTA_MASS is not allowed (must be <= 0)
    crate::common::assert_validation_field(&Opm::from_kvn(kvn).unwrap_err(), "DeltaMassZ");
}

#[test]
fn maneuver_delta_mass_negative() {
    // XSD: deltamassTypeZ - negative values are valid (mass loss)
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 3000.000000 [kg]
MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = -100.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let man = &opm.body.segment.data.maneuver_parameters[0];
    assert_eq!(man.man_delta_mass.value, -100.0);
}
