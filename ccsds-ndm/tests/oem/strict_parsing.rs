use crate::common::{mutated, mutated_once};
use crate::{KVN_FIXTURES, XML};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::Ndm;

#[test]
fn kvn_rejects_unknown_duplicate_reordered_malformed_and_misplaced_content() {
    let source = KVN_FIXTURES[0];
    let object_name = source
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_NAME"))
        .unwrap();
    let object_id = source
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_ID"))
        .unwrap();
    for (label, invalid) in [
        (
            "duplicate keyword",
            mutated(
                source,
                object_name,
                &format!("{object_name}\n{object_name}"),
            ),
        ),
        (
            "reordered keywords",
            mutated(
                source,
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            mutated(
                source,
                object_name,
                &format!("{object_name}\nUNKNOWN = value"),
            ),
        ),
        (
            "misplaced comment",
            mutated(
                source,
                object_name,
                &format!("{object_name}\nCOMMENT misplaced"),
            ),
        ),
        (
            "trailing content",
            format!("{source}\nUNKNOWN TRAILING CONTENT"),
        ),
        (
            "non-ASCII content",
            mutated(source, object_name, &format!("{object_name} €")),
        ),
        (
            "lone carriage return",
            mutated(source, "OBJECT_NAME", "OBJECT\r_NAME"),
        ),
        (
            "seventeen-digit fixed number",
            mutated_once(source, "2789.619", "1.2345678901234567"),
        ),
        (
            "floating point without decimal mantissa",
            mutated_once(source, "2789.619", "1e3"),
        ),
    ] {
        assert!(Oem::from_kvn(&invalid).is_err(), "accepted {label}");
    }

    let crlf = mutated(source, "\n", "\r\n");
    assert_eq!(
        Oem::from_kvn(&crlf).expect("CRLF should parse"),
        Oem::from_kvn(source).expect("LF should parse")
    );
    for (label, input) in [
        ("CR", mutated(source, "\n", "\r")),
        ("LFCR", mutated(source, "\n", "\n\r")),
    ] {
        assert_eq!(
            Oem::from_kvn(&input).unwrap_or_else(|error| panic!("{label} should parse: {error}")),
            Oem::from_kvn(source).unwrap()
        );
    }
}

#[test]
fn kvn_comment_separator_is_not_part_of_the_value() {
    let source = mutated_once(
        KVN_FIXTURES[0],
        "CCSDS_OEM_VERS = 3.0",
        "CCSDS_OEM_VERS = 3.0\nCOMMENT ",
    );
    assert_eq!(Oem::from_kvn(&source).unwrap().header.comment, vec![""]);

    // A producer that omits the separator entirely is still read as an empty comment, matching
    // the other ODM families. Generation always writes the normative `COMMENT ` spelling.
    let bare = mutated_once(&source, "COMMENT \n", "COMMENT\n");
    assert_eq!(Oem::from_kvn(&bare).unwrap().header.comment, vec![""]);
}

#[test]
fn xml_rejects_wrong_envelope_unknown_duplicate_reordered_and_trailing_content() {
    let source = XML;
    let object_name = "<OBJECT_NAME>MARS GLOBAL SURVEYOR</OBJECT_NAME>";
    let object_id = "<OBJECT_ID>2021-028A</OBJECT_ID>";
    assert!(source.contains(&format!("{object_name}\n{object_id}")));
    for (label, invalid) in [
        (
            "wrong root",
            mutated(&mutated(source, "<oem ", "<omm "), "</oem>", "</omm>"),
        ),
        (
            "unknown root attribute",
            mutated(source, "<oem ", "<oem unexpected=\"value\" "),
        ),
        (
            "unknown element",
            mutated(source, "<metadata>", "<metadata><UNKNOWN>value</UNKNOWN>"),
        ),
        (
            "unknown container attribute",
            mutated(source, "<metadata>", "<metadata unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            mutated_once(source, "<X>", "<X unexpected=\"value\">"),
        ),
        (
            "units on a non-unit element",
            mutated_once(source, "<OBJECT_NAME>", "<OBJECT_NAME units=\"km\">"),
        ),
        (
            "duplicate element",
            mutated(source, object_name, &format!("{object_name}{object_name}")),
        ),
        (
            "reordered elements",
            mutated(
                source,
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        ("trailing element", format!("{source}<junk/>")),
        (
            "document type",
            mutated_once(source, "<oem ", "<!DOCTYPE oem><oem "),
        ),
    ] {
        assert!(Oem::from_xml(&invalid).is_err(), "accepted {label}");
    }
}

#[test]
fn xml_declaration_is_optional_but_must_lead_the_document() {
    const DECLARATION: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    let without = XML
        .strip_prefix(DECLARATION)
        .expect("fixture starts with the declaration");

    // Producers that omit the declaration, or spell it differently, are still read.
    let baseline = Oem::from_xml(XML).unwrap();
    assert_eq!(Oem::from_xml(without).unwrap(), baseline);
    assert_eq!(
        Oem::from_xml(&format!("<?xml version=\"1.0\"?>\n{without}")).unwrap(),
        baseline
    );
    // A leading byte-order mark is tolerated ahead of the declaration.
    assert_eq!(Oem::from_xml(&format!("\u{feff}{XML}")).unwrap(), baseline);

    // Only the position is normative: a declaration cannot follow content.
    crate::common::assert_invalid_format(
        &Oem::from_xml(&format!("<!-- lead-in -->\n{XML}")).unwrap_err(),
        "an XML declaration, when present, must begin the document",
    );
}

#[test]
fn xml_contextual_epoch_fields_reject_invalid_values() {
    for (needle, replacement) in [
        (
            "<START_TIME>2019-12-18T12:00:00.331</START_TIME>",
            "<START_TIME>2023-02-29T12:00:00</START_TIME>",
        ),
        (
            "<USEABLE_START_TIME>2019-12-18T12:10:00.331</USEABLE_START_TIME>",
            "<USEABLE_START_TIME>+</USEABLE_START_TIME>",
        ),
        ("<EPOCH>2019-12-18T12:00:00.331</EPOCH>", "<EPOCH>+</EPOCH>"),
        ("<EPOCH>2019-12-28T22:28:00.331</EPOCH>", "<EPOCH>.</EPOCH>"),
    ] {
        let invalid = mutated_once(XML, needle, replacement);
        assert!(
            Oem::from_xml(&invalid).is_err(),
            "accepted invalid contextual epoch replacement {replacement:?}"
        );
    }
}

/// An ephemeris record occupies exactly one line (ODM 7.3.7), so a line holding more than one
/// record must be rejected rather than silently re-read as several records.
#[test]
fn kvn_rejects_ephemeris_records_packed_onto_one_line() {
    let source = KVN_FIXTURES[0];
    let record = source
        .lines()
        .find(|line| line.starts_with("2019-12-18T12:00:00.331"))
        .unwrap();
    let acceleration = format!("{record} 1.0 2.0 3.0");

    for (label, packed) in [
        ("two six-component records", format!("{record} {record}")),
        (
            "two nine-component records",
            format!("{acceleration} {acceleration}"),
        ),
        (
            "record followed by a bare epoch",
            format!("{record} 2019-12-18T12:00:30.331"),
        ),
    ] {
        let error = Oem::from_kvn(&mutated(source, record, &packed)).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

/// A malformed component must be diagnosed as a bad number rather than as a short record, and
/// the padding forms that real producers emit must keep parsing.
#[test]
fn kvn_ephemeris_records_tolerate_padding_and_name_malformed_components() {
    let source = KVN_FIXTURES[0];
    let record = source
        .lines()
        .find(|line| line.starts_with("2019-12-18T12:00:00.331"))
        .unwrap();
    let expected = Oem::from_kvn(source).expect("fixture should parse");

    let padded = format!("{record}   ");
    assert_eq!(
        Oem::from_kvn(&mutated(source, record, &padded)).expect("trailing spaces should parse"),
        expected,
        "trailing spaces changed the parsed model"
    );
    // A tab is not a KVN blank; the strict pass rejects it as non-printable before parsing.
    let tabbed = format!("{record}\t");
    let error = Oem::from_kvn(&mutated(source, record, &tabbed)).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");

    let no_final_newline = source.trim_end_matches('\n');
    assert_eq!(
        Oem::from_kvn(no_final_newline).expect("missing final newline should parse"),
        expected,
    );
    let padded_final_record = format!("{no_final_newline}   ");
    assert_eq!(
        Oem::from_kvn(&padded_final_record).expect("padded final record should parse"),
        expected,
    );

    let large_decimal = mutated_once(source, "-280.045", "3000000000.0");
    Oem::from_kvn(&large_decimal).expect("large decimal-form values should parse");

    for malformed in ["1.2345678901234567", "2147483648", "1e3", "1.", "nan"] {
        let invalid = mutated_once(source, "-280.045", malformed);
        let error = Oem::from_kvn(&invalid)
            .expect_err(&format!("accepted malformed component {malformed}"))
            .to_string();
        assert!(
            error.contains("Invalid ODM number"),
            "component {malformed} was misdiagnosed as {error}"
        );
    }
}
