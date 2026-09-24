use crate::common::{mutated, mutated_once};
use crate::{KVN_FIXTURES, XML};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::Ndm;

#[test]
fn xml_optional_fields_keep_their_declared_types() {
    for (tag, value, code) in [
        ("INTERPOLATION_DEGREE", "", "parse.xml.syntax"),
        ("INTERPOLATION_DEGREE", "n/a", "parse.xml.syntax"),
        ("X_DDOT", "", "parse.xml.syntax"),
        ("X_DDOT", "n/a", "parse.xml.syntax"),
        ("USEABLE_START_TIME", "", "validation.invalid_value"),
    ] {
        let start = XML.find(&format!("<{tag}")).unwrap();
        let end = start + XML[start..].find(&format!("</{tag}>")).unwrap() + tag.len() + 3;
        let invalid = format!("{}<{tag}>{value}</{tag}>{}", &XML[..start], &XML[end..]);
        let error = Oem::from_xml(&invalid).unwrap_err();
        assert_eq!(error.code(), Some(code), "{tag}={value:?}: {error}");
    }
    // Optional strings are XML strings, not JSON or a nullable sentinel.
    for value in ["n/a", "&quot;CUSTOM&quot;", "", " ", "\t\n", "  HERMITE  "] {
        let input = mutated_once(
            XML,
            "<INTERPOLATION>HERMITE</INTERPOLATION>",
            &format!("<INTERPOLATION>{value}</INTERPOLATION>"),
        );
        let expected = value.replace("&quot;", "\"");
        let parsed = Oem::from_xml(&input).unwrap();
        assert_eq!(
            parsed.body.segment[0].metadata.interpolation.as_deref(),
            Some(expected.as_str())
        );
        assert_eq!(Oem::from_xml(&parsed.to_xml().unwrap()).unwrap(), parsed);
    }
    let mut message = Oem::from_xml(XML).unwrap();
    message.header.classification = Some("\"quoted classification\"".into());
    message.header.message_id = Some("n/a".into());
    assert_eq!(Oem::from_xml(&message.to_xml().unwrap()).unwrap(), message);
}

#[test]
fn kvn_interpolation_degree_obeys_signed_integer_grammar() {
    let source = KVN_FIXTURES[0];
    let signed = mutated(
        source,
        "INTERPOLATION_DEGREE = 7",
        "INTERPOLATION_DEGREE = +0007",
    );
    assert_eq!(
        Oem::from_kvn(&signed).unwrap(),
        Oem::from_kvn(source).unwrap()
    );
    // The degree is positive (Table 5-3) and a KVN integer is a signed 32-bit value (7.5.4).
    for value in ["-1", "0", "2147483648"] {
        let invalid = mutated(
            source,
            "INTERPOLATION_DEGREE = 7",
            &format!("INTERPOLATION_DEGREE = {value}"),
        );
        let error = Oem::from_kvn(&invalid).unwrap_err();
        let kvn = crate::common::kvn_parse_error(&error)
            .unwrap_or_else(|| panic!("{value}: expected a KVN parse error, got {error}"));
        assert_eq!((kvn.line, kvn.column), (15, 24), "{value}: {error}");
    }
}

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
            format!("{source}\nUNKNOWN TRAILING CONTENT\n"),
        ),
        (
            "non-ASCII content",
            mutated(source, object_name, &format!("{object_name} €")),
        ),
        (
            "floating point without decimal mantissa",
            mutated_once(source, "2789.619", "1e3"),
        ),
    ] {
        let error = Oem::from_kvn(&invalid).unwrap_err();
        if label == "trailing content" {
            crate::common::assert_invalid_epoch(&error, "UNKNOWN");
        } else {
            assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
        }
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
        let error = Oem::from_xml(&invalid).unwrap_err();
        assert_eq!(error.code(), Some("parse.xml.syntax"), "{label}: {error}");
    }
}

#[test]
fn xml_declaration_is_the_exact_first_line() {
    // ODM 8.2 and NDM/XML 4.2: the first line is exactly the version-1.0, UTF-8 declaration.
    const DECLARATION: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    let without = XML
        .strip_prefix(DECLARATION)
        .expect("fixture starts with the declaration");
    let baseline = Oem::from_xml(XML).unwrap();
    // A byte-order mark is an encoding signature, not part of the first line.
    assert_eq!(Oem::from_xml(&format!("\u{feff}{XML}")).unwrap(), baseline);
    assert_eq!(
        Oem::from_xml(&XML.replacen('\n', "\r\n", 1)).unwrap(),
        baseline
    );

    for input in [
        without.to_owned(),
        format!("<?xml version=\"1.0\"?>\n{without}"),
        format!("<?xml version='1.0' encoding='UTF-8'?>\n{without}"),
        format!("<!-- lead-in -->\n{XML}"),
        format!("{}{without}", DECLARATION.trim_end()),
    ] {
        crate::common::assert_invalid_format(
            &Oem::from_xml(&input).unwrap_err(),
            "the first line of an XML instantiation must be exactly \
             <?xml version=\"1.0\" encoding=\"UTF-8\"?>",
        );
    }
}

#[test]
fn xml_contextual_epoch_fields_reject_invalid_values() {
    for (field, needle, replacement) in [
        (
            "START_TIME",
            "<START_TIME>2019-12-18T12:00:00.331</START_TIME>",
            "<START_TIME>2023-02-29T12:00:00</START_TIME>",
        ),
        (
            "USEABLE_START_TIME",
            "<USEABLE_START_TIME>2019-12-18T12:10:00.331</USEABLE_START_TIME>",
            "<USEABLE_START_TIME>+</USEABLE_START_TIME>",
        ),
        (
            "stateVector EPOCH",
            "<EPOCH>2019-12-18T12:00:00.331</EPOCH>",
            "<EPOCH>+</EPOCH>",
        ),
        (
            "covarianceMatrix EPOCH",
            "<EPOCH>2019-12-28T22:28:00.331</EPOCH>",
            "<EPOCH>.</EPOCH>",
        ),
    ] {
        let invalid = mutated_once(XML, needle, replacement);
        crate::common::assert_validation_field(&Oem::from_xml(&invalid).unwrap_err(), field);
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

    // ODM 7.3.7 terminates every line; parsing leniently accepts an unterminated last line.
    let no_final_newline = source.trim_end_matches('\n');
    assert_eq!(
        Oem::from_kvn(no_final_newline).expect("unterminated final line should parse"),
        expected,
    );
    let padded_final_record = format!("{no_final_newline}   \n");
    assert_eq!(
        Oem::from_kvn(&padded_final_record).expect("padded final record should parse"),
        expected,
    );

    // Parsing drops the 16-digit cap of 7.5.6/7.5.7b and the 7.5.4 integer range for data
    // values, so 17-digit shortest round-trip doubles and large integers still parse.
    for (lenient, value) in [
        ("3000000000.0", 3.0e9),
        ("3000000000", 3.0e9),
        ("-2757.3016318893897", -2757.3016318893897),
        ("1.2345678901234567e+03", 1234.5678901234567),
    ] {
        let parsed = Oem::from_kvn(&mutated_once(source, "-280.045", lenient))
            .unwrap_or_else(|error| panic!("{lenient} should parse: {error}"));
        let state = &parsed.body.segment[0].data.state_vector[0];
        assert!(
            [state.x.value, state.y.value, state.z.value].contains(&value),
            "{lenient}"
        );
    }

    for malformed in ["1e3", "12.5e3", "1.", "nan"] {
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

#[test]
fn kvn_accepts_blank_lines_and_trailing_blanks_anywhere() {
    // ODM 7.3.5 permits blank lines at any position; 7.4.7 makes trailing white space
    // insignificant. Neither changes the parsed message.
    let with_covariance = KVN_FIXTURES[2];
    let expected = Oem::from_kvn(with_covariance).unwrap();
    let covariance_stop = with_covariance.find("COVARIANCE_STOP").unwrap();
    let variants = [
        mutated_once(with_covariance, "\nORIGINATOR", "\n\n  \nORIGINATOR"),
        mutated_once(with_covariance, "\nMESSAGE_ID", "\n\nMESSAGE_ID"),
        format!(
            "{}\n{}",
            &with_covariance[..covariance_stop],
            &with_covariance[covariance_stop..]
        ),
        with_covariance
            .lines()
            .map(|line| format!("{line}  \r\n"))
            .collect(),
    ];
    for variant in variants {
        assert_eq!(Oem::from_kvn(&variant).unwrap(), expected, "{variant}");
    }
}

#[test]
fn kvn_empty_optional_values_are_absent() {
    // ODM 7.5.1 requires non-empty values only for mandatory keywords.
    let source = KVN_FIXTURES[0];
    let mut expected = Oem::from_kvn(source).unwrap();
    let metadata = &mut expected.body.segment[0].metadata;
    metadata.useable_start_time = None;
    metadata.useable_stop_time = None;
    metadata.interpolation = None;
    metadata.interpolation_degree = None;
    let input = mutated_once(
        source,
        "USEABLE_START_TIME = 2019-12-18T12:10:00.331\nUSEABLE_STOP_TIME = 2019-12-28T21:23:00.331\n\
         STOP_TIME = 2019-12-28T21:28:00.331\nINTERPOLATION = HERMITE\nINTERPOLATION_DEGREE = 7",
        "USEABLE_START_TIME =\nUSEABLE_STOP_TIME =  \nSTOP_TIME = 2019-12-28T21:28:00.331\n\
         INTERPOLATION =\nINTERPOLATION_DEGREE =",
    );
    let input = mutated_once(
        &input,
        "REF_FRAME = EME2000\n",
        "REF_FRAME = EME2000\nREF_FRAME_EPOCH =\n",
    );
    assert_eq!(Oem::from_kvn(&input).unwrap(), expected);
}

#[test]
fn kvn_accepts_an_empty_covariance_section() {
    // Annex A2.5.3 row 30 makes covariance lines optional inside the covariance block.
    let source = KVN_FIXTURES[2];
    let start = source.find("COVARIANCE_START").unwrap();
    let stop = source.find("COVARIANCE_STOP").unwrap();
    let input = format!("{}COVARIANCE_START\n{}", &source[..start], &source[stop..]);
    let parsed = Oem::from_kvn(&input).unwrap();
    assert!(parsed.body.segment[0].data.covariance_matrix.is_empty());
}

#[test]
fn xml_namespace_declarations_and_schema_hints_are_not_message_attributes() {
    // XML Namespaces: declarations are not attributes; XSD admits schema-location hints on any
    // element. NDM/XML 4.3.3 fixes only the presence of xmlns:xsi on the root.
    let expected = Oem::from_xml(XML).unwrap();
    for (from, to) in [
        (
            "<oem ",
            "<oem xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\" xmlns=\"\" ",
        ),
        (
            "<oem ",
            "<oem xmlns:x=\"http://www.w3.org/2001/XMLSchema-instance\" \
             x:schemaLocation=\"urn:ccsds:schema:ndmxml oem.xsd\" ",
        ),
        ("<header>", "<header xmlns:foo=\"urn:foo\" xmlns=\"\">"),
        (
            "<header>",
            "<header xsi:noNamespaceSchemaLocation=\"oem.xsd\">",
        ),
    ] {
        let input = mutated_once(XML, from, to);
        assert_eq!(Oem::from_xml(&input).unwrap(), expected, "{to}");
    }
}

#[test]
fn xml_accepts_the_qualified_schema_form() {
    // NDM/XML 4.3.5 and ODM 8.3.3: the qualified set prefixes every element with the NDM
    // namespace. The books show the root both prefixed and unprefixed.
    let expected = Oem::from_xml(XML).unwrap();
    let qualified = XML
        .replace("</", "\u{0}")
        .replace('<', "<ndm:")
        .replace('\u{0}', "</ndm:")
        .replace("<ndm:?xml", "<?xml")
        .replace("<ndm:!--", "<!--")
        .replace(
            "<ndm:oem ",
            "<ndm:oem xmlns:ndm=\"urn:ccsds:schema:ndmxml\" ",
        );
    assert_eq!(Oem::from_xml(&qualified).unwrap(), expected);
    let unprefixed_root =
        qualified
            .replacen("<ndm:oem ", "<oem ", 1)
            .replacen("</ndm:oem>", "</oem>", 1);
    assert_eq!(Oem::from_xml(&unprefixed_root).unwrap(), expected);

    let mixed = mutated_once(&qualified, "<ndm:header>", "<header>");
    let mixed = mutated_once(&mixed, "</ndm:header>", "</header>");
    crate::common::assert_invalid_format(
        &Oem::from_xml(&mixed).unwrap_err(),
        "invalid OEM XML sequence: qualified and unqualified NDM/XML elements cannot be mixed",
    );
    let foreign = mutated_once(XML, "<oem ", "<oem xmlns:o=\"urn:other\" ");
    let foreign = mutated_once(&foreign, "<header>", "<o:header>");
    let foreign = mutated_once(&foreign, "</header>", "</o:header>");
    crate::common::assert_invalid_format(
        &Oem::from_xml(&foreign).unwrap_err(),
        "invalid OEM XML sequence: element 'o:header' is not in the NDM/XML namespace",
    );
}

#[test]
fn xml_root_must_declare_the_schema_instance_namespace() {
    // NDM/XML 4.3.3 and ODM 8.3.3.
    let root_end = XML.find("id=\"CCSDS_OEM_VERS\"").unwrap();
    let input = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<oem {}",
        &XML[root_end..]
    );
    crate::common::assert_invalid_format(
        &Oem::from_xml(&input).unwrap_err(),
        "the OEM root element must declare \
         xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"",
    );
}

#[test]
fn kvn_rejects_overlong_lines_and_short_covariance_rows() {
    // ODM 7.3.2 limits lines to 254 characters; 5.2.5.4 fixes the lower-triangular row widths.
    let source = KVN_FIXTURES[2];
    let comment = format!("COMMENT {}", "x".repeat(246));
    assert_eq!(comment.len(), 254);
    let at_limit = mutated_once(
        source,
        "COMMENT This block",
        &format!("{comment}\nCOMMENT This block"),
    );
    Oem::from_kvn(&at_limit).unwrap();
    let overlong = mutated_once(
        source,
        "COMMENT This block",
        &format!("{comment}x\nCOMMENT This block"),
    );
    let error = Oem::from_kvn(&overlong).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");

    let short_row = mutated_once(source, "4.6189273e-04 6.7824216e-04\n", "4.6189273e-04\n");
    let error = Oem::from_kvn(&short_row).unwrap_err();
    let kvn = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error}"));
    assert_eq!(kvn.line, 27, "{error}");
}

#[test]
fn xml_explicit_units_must_be_the_fixed_oem_units() {
    // ODM 8.13.6: XML units are the KVN units; OEM positions are km.
    let explicit = mutated_once(XML, "<X>2789.6</X>", "<X units=\"km\">2789.6</X>");
    assert_eq!(
        Oem::from_xml(&explicit).unwrap(),
        Oem::from_xml(XML).unwrap()
    );
    for units in ["m", "KM"] {
        let wrong = mutated_once(
            XML,
            "<X>2789.6</X>",
            &format!("<X units=\"{units}\">2789.6</X>"),
        );
        let error = Oem::from_xml(&wrong).unwrap_err();
        assert_eq!(error.code(), Some("parse.xml.syntax"), "{units}: {error}");
    }
}

/// Returns the KVN syntax message behind a strict-parsing refusal.
#[track_caller]
fn kvn_message(input: &str) -> String {
    let error = Oem::from_kvn(input).unwrap_err();
    crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error}"))
        .message
        .clone()
}

#[test]
fn kvn_record_structure_rejections_name_their_reason() {
    // Table 5-2 and 5-3 fix the header and segment layout; 7.8.9 places comments.
    let source = KVN_FIXTURES[0];
    let covariance = KVN_FIXTURES[2];
    let first_record =
        "2019-12-18T12:00:00.331 2789.619 -280.045 -1746.755 4.73372 -2.49586 -1.04195\n";
    let header_end = "ORIGINATOR = NASA/JPL\n";
    for (input, message) in [
        (
            mutated_once(source, first_record, &format!("META_START\n{first_record}")),
            "unexpected META_START",
        ),
        (
            mutated_once(source, first_record, &format!("META_STOP\n{first_record}")),
            "unexpected META_STOP",
        ),
        (
            mutated_once(source, "META_STOP\n", "META_STOP\nCOVARIANCE_START\n"),
            "COVARIANCE_START must follow ephemeris records",
        ),
        (
            mutated_once(
                covariance,
                "COVARIANCE_STOP",
                "COVARIANCE_STOP\nCOVARIANCE_START\nCOVARIANCE_STOP",
            ),
            "COVARIANCE_START must follow ephemeris records",
        ),
        (
            mutated_once(
                covariance,
                "-3.0302350e-07 -4.8783858e-07 3.4302008e-07 1.7581520e-10 1.0077514e-10 \
                 6.2244443e-10\nCOVARIANCE_STOP",
                "COVARIANCE_STOP",
            ),
            "COVARIANCE_STOP must follow a complete covariance matrix",
        ),
        (
            mutated_once(
                source,
                header_end,
                &format!("{header_end}UNKNOWN = value\n"),
            ),
            "unknown OEM header keyword",
        ),
        (
            mutated_once(
                source,
                "CREATION_DATE = 1996-11-04T17:22:31\nORIGINATOR = NASA/JPL",
                "ORIGINATOR = NASA/JPL\nCREATION_DATE = 1996-11-04T17:22:31",
            ),
            "duplicate or out-of-order OEM header keyword",
        ),
        (
            mutated_once(
                source,
                "CCSDS_OEM_VERS = 3.0\nCREATION_DATE = 1996-11-04T17:22:31",
                "CREATION_DATE = 1996-11-04T17:22:31\nCCSDS_OEM_VERS = 3.0",
            ),
            "CCSDS_OEM_VERS must be the first record",
        ),
        (
            mutated_once(covariance, "COV_REF_FRAME = EME2000", "REF_FRAME = EME2000"),
            "unexpected covariance keyword",
        ),
        (
            mutated_once(
                source,
                first_record,
                &format!("OBJECT_NAME = X\n{first_record}"),
            ),
            "assignments are not allowed in OEM ephemeris records",
        ),
        (
            source[..source.find("META_START").unwrap()].to_owned(),
            "incomplete OEM document",
        ),
        (
            mutated_once(source, header_end, &format!("{header_end}COMMENT late\n")),
            "COMMENT is not at the beginning of an allowed OEM block",
        ),
        (
            mutated_once(
                source,
                first_record,
                &format!("{first_record}COMMENT inside\n"),
            ),
            "COMMENT is not at the beginning of an allowed OEM block",
        ),
        (
            mutated_once(
                covariance,
                "EPOCH = 2019-12-29T21:00:00",
                "COMMENT between\nEPOCH = 2019-12-29T21:00:00",
            ),
            "COMMENT is not at the beginning of an allowed OEM block",
        ),
    ] {
        assert_eq!(kvn_message(&input), message, "{input}");
    }
}

#[test]
fn kvn_numbers_stay_within_the_double_range() {
    // ODM 7.5.7e bounds floating-point values by the double range; subnormals remain valid.
    let source = KVN_FIXTURES[2];
    for (from, to) in [
        ("-2432.166", "1.0E999"),
        ("-2432.166", "-1.0E999"),
        ("-2432.166", "1.0E-999"),
        ("3.3313494e-04", "9.9E400"),
    ] {
        assert_eq!(
            kvn_message(&mutated_once(source, from, to)),
            "Invalid ODM number",
            "{to}"
        );
    }
    let subnormal = Oem::from_kvn(&mutated_once(source, "-2432.166", "4.9E-324")).unwrap();
    let x = subnormal.body.segment[0].data.state_vector[0].x.value;
    assert!(x > 0.0 && !x.is_normal(), "{x}");
    let zero = Oem::from_kvn(&mutated_once(source, "-2432.166", "0.0E-999")).unwrap();
    assert_eq!(zero.body.segment[0].data.state_vector[0].x.value, 0.0);
    let padded = Oem::from_kvn(&mutated_once(source, "-2432.166", "+00000000000000000001"))
        .expect("7.5.4 permits leading zeroes without the real-number digit cap");
    assert_eq!(padded.body.segment[0].data.state_vector[0].x.value, 1.0);
}

#[test]
fn kvn_covariance_section_may_hold_only_comments() {
    // 7.8.9 admits comments right after COVARIANCE_START and A2.5.3 makes the matrices
    // optional. With no matrix to carry them, the comments join the data comments.
    let source = KVN_FIXTURES[2];
    let start = source.find("COVARIANCE_START").unwrap();
    let stop = source.find("COVARIANCE_STOP").unwrap();
    let input = format!(
        "{}COVARIANCE_START\nCOMMENT no covariance available\n{}",
        &source[..start],
        &source[stop..]
    );
    let data = &Oem::from_kvn(&input).unwrap().body.segment[0].data;
    assert!(data.covariance_matrix.is_empty());
    assert_eq!(
        data.comment.last().map(String::as_str),
        Some("no covariance available")
    );
}

#[test]
fn kvn_empty_cov_ref_frame_is_absent() {
    // ODM 7.5.1, as for the other optional keywords.
    let input = mutated_once(
        KVN_FIXTURES[2],
        "COV_REF_FRAME = EME2000",
        "COV_REF_FRAME =",
    );
    let parsed = Oem::from_kvn(&input).unwrap();
    assert_eq!(
        parsed.body.segment[0].data.covariance_matrix[0].cov_ref_frame,
        None
    );
}

#[test]
fn kvn_listed_frames_use_a_single_case() {
    // 7.5.3 applies to the frames listed in 3.2.3.3 and, for COV_REF_FRAME, 3.2.4.11.
    for frame in ["Eme2000", "Rsw", "Rtn", "Tnw"] {
        let input = mutated_once(
            KVN_FIXTURES[2],
            "COV_REF_FRAME = EME2000",
            &format!("COV_REF_FRAME = {frame}"),
        );
        crate::common::assert_validation_field(
            &Oem::from_kvn(&input).unwrap_err(),
            "COV_REF_FRAME",
        );
    }
    for frame in ["rtn", "TNW"] {
        let input = mutated_once(
            KVN_FIXTURES[2],
            "COV_REF_FRAME = EME2000",
            &format!("COV_REF_FRAME = {frame}"),
        );
        Oem::from_kvn(&input).unwrap_or_else(|error| panic!("{frame}: {error}"));
    }
}

#[test]
fn interpolation_requires_its_degree_in_both_notations() {
    // Table 5-3: INTERPOLATION_DEGREE must be used if INTERPOLATION is used.
    let degree_element = "<INTERPOLATION_DEGREE>7</INTERPOLATION_DEGREE>";
    for input in [
        mutated_once(KVN_FIXTURES[0], "INTERPOLATION_DEGREE = 7\n", ""),
        mutated_once(
            KVN_FIXTURES[0],
            "INTERPOLATION_DEGREE = 7\n",
            "INTERPOLATION_DEGREE =\n",
        ),
    ] {
        let error = Oem::from_kvn(&input).unwrap_err();
        assert_eq!(
            error.code(),
            Some("validation.missing_required_field"),
            "{error}"
        );
    }
    let error = Oem::from_xml(&mutated_once(XML, degree_element, "")).unwrap_err();
    assert_eq!(
        error.code(),
        Some("validation.missing_required_field"),
        "{error}"
    );
}

#[test]
fn every_mandatory_keyword_is_required_in_both_notations() {
    // Tables 5-2 and 5-3 mark these keywords mandatory.
    let kvn = KVN_FIXTURES[2];
    for keyword in [
        "CREATION_DATE",
        "ORIGINATOR",
        "OBJECT_NAME",
        "OBJECT_ID",
        "CENTER_NAME",
        "REF_FRAME",
        "TIME_SYSTEM",
        "START_TIME",
        "STOP_TIME",
    ] {
        let line = kvn
            .lines()
            .find(|line| line.split('=').next().unwrap().trim() == keyword)
            .unwrap();
        let error = Oem::from_kvn(&mutated_once(kvn, &format!("{line}\n"), "")).unwrap_err();
        assert_eq!(
            error.code(),
            Some("validation.missing_required_field"),
            "KVN {keyword}: {error}"
        );
        assert!(
            error.to_string().contains(keyword),
            "KVN {keyword}: {error}"
        );

        let start = XML.find(&format!("<{keyword}>")).unwrap();
        let end = start + XML[start..].find('\n').unwrap();
        let error = Oem::from_xml(&format!("{}{}", &XML[..start], &XML[end..])).unwrap_err();
        assert_eq!(
            error.code(),
            Some("parse.xml.syntax"),
            "XML {keyword}: {error}"
        );
        assert!(
            error.to_string().contains(keyword),
            "XML {keyword}: {error}"
        );
    }
}

#[test]
fn classification_is_read_between_the_version_and_creation_date() {
    // Table 5-2 places the optional CLASSIFICATION after the header comments.
    let kvn = mutated_once(
        KVN_FIXTURES[2],
        "CREATION_DATE",
        "CLASSIFICATION = SBU\nCREATION_DATE",
    );
    let xml = mutated_once(
        XML,
        "<CREATION_DATE>",
        "<CLASSIFICATION>SBU</CLASSIFICATION>\n<CREATION_DATE>",
    );
    for message in [Oem::from_kvn(&kvn).unwrap(), Oem::from_xml(&xml).unwrap()] {
        assert_eq!(message.header.classification.as_deref(), Some("SBU"));
        assert_eq!(Oem::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    }
    let late = mutated_once(
        KVN_FIXTURES[2],
        "MESSAGE_ID",
        "CLASSIFICATION = SBU\nMESSAGE_ID",
    );
    assert_eq!(
        Oem::from_kvn(&late).unwrap_err().code(),
        Some("parse.kvn.syntax")
    );
}

#[test]
fn oem_1_0_is_read_as_kvn_only_without_later_content() {
    // 502.0-B-1 defines OEM 1.0 in KVN only, without the header's CLASSIFICATION and
    // MESSAGE_ID, REF_FRAME_EPOCH, accelerations or covariance. It is read but not written.
    let g11 = include_str!("../../data/kvn/oem_g11.kvn");
    let kvn = mutated_once(g11, "CCSDS_OEM_VERS = 3.0", "CCSDS_OEM_VERS = 1.0");
    let message = Oem::from_kvn(&kvn).unwrap();
    assert_eq!(message.version, "1.0");
    for error in [message.to_kvn().unwrap_err(), message.to_xml().unwrap_err()] {
        assert_eq!(
            error.code(),
            Some("generation.unsupported_output_version"),
            "{error}"
        );
    }

    let first_state = g11.lines().find(|line| line.starts_with("2019-")).unwrap();
    for (label, input, path) in [
        (
            "MESSAGE_ID",
            mutated_once(&kvn, "ORIGINATOR = NASA/JPL\n", "ORIGINATOR = NASA/JPL\nMESSAGE_ID = X\n"),
            "header.message_id",
        ),
        (
            "REF_FRAME_EPOCH",
            mutated_once(&kvn, "TIME_SYSTEM", "REF_FRAME_EPOCH = 2000-01-01T00:00:00\nTIME_SYSTEM"),
            "body.segment[0].metadata.ref_frame_epoch",
        ),
        (
            "acceleration",
            mutated_once(&kvn, first_state, &format!("{first_state} 1.0 2.0 3.0")),
            "body.segment[0].data.state_vector[0]",
        ),
        (
            "covariance",
            format!(
                "{kvn}COVARIANCE_START\nEPOCH = {}\n1\n0 1\n0 0 1\n0 0 0 1\n0 0 0 0 1\n0 0 0 0 0 1\nCOVARIANCE_STOP\n",
                first_state.split(' ').next().unwrap()
            ),
            "body.segment[1].data.covariance_matrix",
        ),
    ] {
        let error = Oem::from_kvn(&input).unwrap_err();
        assert_eq!(error.field_path().as_deref(), Some(path), "{label}: {error}");
    }

    let xml = mutated_once(XML, "version=\"3.0\"", "version=\"1.0\"");
    assert_eq!(
        Oem::from_xml(&xml).unwrap_err().code(),
        Some("parse.unsupported_input_version")
    );
}

#[test]
fn producer_style_kvn_parses_and_writes_back_within_the_digit_limit() {
    // Real OEM producers commonly align keywords, use CRLF, write 17-digit shortest round-trip
    // doubles and omit the final line terminator. Reading accepts all of it; see "More lenient
    // than the book" in docs/conformance/oem-3.0.md.
    // The end-of-file hook terminates committed fixtures, so drop the final line ending here.
    let source = include_str!("fixtures/producer_style.oem").trim_end_matches(['\r', '\n']);
    assert!(source.contains("\r\n"));
    let message = Oem::from_kvn(source).unwrap();
    let data = &message.body.segment[0].data;
    assert_eq!(data.state_vector.len(), 3);
    assert_eq!(data.state_vector[0].x.value, -2757.3016318893897);
    assert_eq!(data.covariance_matrix[0].cy_x.value, 0.0);
    assert_eq!(Oem::from_xml(&message.to_xml().unwrap()).unwrap(), message);

    // KVN output follows the book: terminated lines and at most 16 significant digits.
    let kvn = message.to_kvn().unwrap();
    assert!(kvn.ends_with('\n'));
    let reparsed = Oem::from_kvn(&kvn).unwrap();
    for (written, read) in reparsed.body.segment[0]
        .data
        .state_vector
        .iter()
        .zip(&data.state_vector)
    {
        for (written, read) in [
            (written.x.value, read.x.value),
            (written.z_dot.value, read.z_dot.value),
        ] {
            assert!(
                (written - read).abs() <= read.abs() * 1e-15,
                "{written} vs {read}"
            );
        }
    }
}

#[test]
fn kvn_time_tag_errors_name_their_line_and_column() {
    // A malformed epoch in a million-record history must be findable.
    for (from, to, line, column) in [
        (
            "2019-12-18T12:01:00.331 2783.419",
            "2019-12-18T12:01:00.33X 2783.419",
            20,
            1,
        ),
        (
            "START_TIME = 2019-12-18T12:00:00.331",
            "START_TIME = 2019-12-18T12:00:00.33X",
            10,
            14,
        ),
    ] {
        let error = Oem::from_kvn(&mutated_once(KVN_FIXTURES[0], from, to)).unwrap_err();
        let kvn = crate::common::kvn_parse_error(&error)
            .unwrap_or_else(|| panic!("expected a located KVN error, got {error}"));
        assert_eq!((kvn.line, kvn.column), (line, column), "{error}");
        assert!(kvn.message.contains("invalid epoch format"), "{error}");
    }
}

#[test]
fn kvn_covariance_epoch_cannot_repeat_before_its_matrix() {
    let input = mutated_once(
        KVN_FIXTURES[2],
        "EPOCH = 2019-12-28T21:29:07.267\n",
        "EPOCH = 2019-12-28T21:29:07.267\nEPOCH = 2019-12-28T21:29:07.267\n",
    );
    let error = Oem::from_kvn(&input).unwrap_err();
    let kvn = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error}"));
    assert_eq!(kvn.line, 25, "{error}");
    assert_eq!(kvn.message, "unexpected covariance keyword", "{error}");
}
