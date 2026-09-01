use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::ParseOptions;

const KVN_FIXTURES: [&str; 3] = [
    include_str!("../data/kvn/oem_g11.kvn"),
    include_str!("../data/kvn/oem_g12.kvn"),
    include_str!("../data/kvn/oem_g13.kvn"),
];
const XML: &str = include_str!("../data/xml/oem_g14.xml");

#[test]
fn shipped_oem_3_fixtures_parse_strictly() {
    for source in KVN_FIXTURES {
        let message = Oem::from_kvn(source).expect("shipped OEM KVN fixture should parse");
        assert_eq!(message.version, "3.0");
    }
    let message = Oem::from_xml(XML).expect("shipped OEM XML fixture should parse");
    assert_eq!(message.version, "3.0");
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
            source.replace(object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered keywords",
            source.replace(
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            source.replace(object_name, &format!("{object_name}\nUNKNOWN = value")),
        ),
        (
            "misplaced comment",
            source.replace(object_name, &format!("{object_name}\nCOMMENT misplaced")),
        ),
        (
            "trailing content",
            format!("{source}\nUNKNOWN TRAILING CONTENT"),
        ),
        (
            "non-ASCII content",
            source.replace(object_name, &format!("{object_name} €")),
        ),
        (
            "lone carriage return",
            source.replace("OBJECT_NAME", "OBJECT\r_NAME"),
        ),
        (
            "seventeen-digit fixed number",
            source.replacen("2789.619", "1.2345678901234567", 1),
        ),
        (
            "floating point without decimal mantissa",
            source.replacen("2789.619", "1e3", 1),
        ),
    ] {
        assert!(Oem::from_kvn(&invalid).is_err(), "accepted {label}");
    }

    let crlf = source.replace('\n', "\r\n");
    assert_eq!(
        Oem::from_kvn(&crlf).expect("CRLF should parse"),
        Oem::from_kvn(source).expect("LF should parse")
    );
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
            source.replace("<oem ", "<omm ").replace("</oem>", "</omm>"),
        ),
        (
            "unknown root attribute",
            source.replace("<oem ", "<oem unexpected=\"value\" "),
        ),
        (
            "unknown element",
            source.replace("<metadata>", "<metadata><UNKNOWN>value</UNKNOWN>"),
        ),
        (
            "unknown container attribute",
            source.replace("<metadata>", "<metadata unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            source.replacen("<X>", "<X unexpected=\"value\">", 1),
        ),
        (
            "units on a non-unit element",
            source.replacen("<OBJECT_NAME>", "<OBJECT_NAME units=\"km\">", 1),
        ),
        (
            "duplicate element",
            source.replace(object_name, &format!("{object_name}{object_name}")),
        ),
        (
            "reordered elements",
            source.replace(
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        ("trailing element", format!("{source}<junk/>")),
        (
            "document type",
            source.replacen("<oem ", "<!DOCTYPE oem><oem ", 1),
        ),
    ] {
        assert!(Oem::from_xml(&invalid).is_err(), "accepted {label}");
    }
}

#[test]
fn input_depth_and_history_limits_are_exact() {
    let message = Oem::from_xml(XML).unwrap();
    let record_count: usize = message
        .body
        .segment
        .iter()
        .map(|segment| segment.data.state_vector.len() + segment.data.covariance_matrix.len())
        .sum();

    Oem::from_xml_with_options(
        XML,
        &ParseOptions::default()
            .with_max_input_bytes(XML.len())
            .with_max_records(record_count),
    )
    .expect("exact input and record limits should pass");

    for options in [
        ParseOptions::default().with_max_input_bytes(XML.len() - 1),
        ParseOptions::default().with_max_records(record_count - 1),
        ParseOptions::default().with_max_xml_depth(1),
    ] {
        assert!(
            Oem::from_xml_with_options(XML, &options).is_err(),
            "undersized resource limit was ignored"
        );
    }
}
