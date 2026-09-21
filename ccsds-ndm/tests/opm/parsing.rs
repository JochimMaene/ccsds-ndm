// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{mutated, mutated_once};

use crate::opm_with_maneuvers;
use ccsds_ndm::error::{DiagnosticNotation, DiagnosticOperation};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::types::CalendarEpoch;
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
    assert!(
        Opm::from_xml(&xml).is_err(),
        "{label} was silently accepted"
    );
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
        (
            "invalid units hidden by nil",
            "<MASS units=\"kg\">",
            "<MASS units=\"km\" nil=\"true\">",
        ),
    ] {
        assert!(source.contains(from), "fixture should contain {from}");
        assert_xml_rejected(label, mutated(&source, from, to));
    }
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
