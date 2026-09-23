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

    // ODM 7.3.7 terminates every line, including the last.
    let no_final_newline = source.trim_end_matches('\n');
    let error = Oem::from_kvn(no_final_newline).unwrap_err();
    let kvn = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error}"));
    assert_eq!(kvn.message, "the final line is not terminated");
    let padded_final_record = format!("{no_final_newline}   \n");
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
