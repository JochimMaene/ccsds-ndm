use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{convert, convert_file, convert_file_with_options, Notation, ParseOptions};

const KVN_FIXTURES: [&str; 4] = [
    include_str!("../data/kvn/opm_g1.kvn"),
    include_str!("../data/kvn/opm_g2.kvn"),
    include_str!("../data/kvn/opm_g3.kvn"),
    include_str!("../data/kvn/opm_g4.kvn"),
];
const XML_FIXTURE: &str = include_str!("../data/xml/opm_g5.xml");

#[test]
fn both_conversion_directions_preserve_the_complete_typed_model() {
    for source in KVN_FIXTURES {
        let expected = Opm::from_kvn(source).expect("KVN fixture should parse");
        let xml = convert(source, Notation::Xml).expect("KVN to XML conversion should work");
        assert_eq!(
            Opm::from_xml(&xml).expect("output XML should parse"),
            expected
        );
    }

    let expected = Opm::from_xml(XML_FIXTURE).expect("XML fixture should parse");
    let kvn = convert(XML_FIXTURE, Notation::Kvn).expect("XML to KVN conversion should work");
    assert_eq!(
        Opm::from_kvn(&kvn).expect("output KVN should parse"),
        expected
    );
}

#[test]
fn xml_to_kvn_rounds_values_to_the_ccsds_digit_limit() {
    let mut message = Opm::from_kvn(KVN_FIXTURES[0]).expect("fixture should parse");
    message.body.segment.data.state_vector.x.value = 1.234_567_890_123_456_7;
    let xml = message.to_xml().expect("XML can represent the f64 exactly");

    let kvn = convert(&xml, Notation::Kvn)
        .expect("finite XML value should round to a conforming KVN value");
    assert!(kvn.contains("X                    = 1.234567890123457e0"));
}

#[test]
fn file_conversion_replaces_the_destination_only_after_success() {
    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let source = directory.path().join("source.opm");
    let destination = directory.path().join("destination.xml");
    std::fs::write(&source, KVN_FIXTURES[0]).expect("source should be written");
    std::fs::write(&destination, b"sentinel").expect("sentinel should be written");

    convert_file(&source, &destination, Notation::Xml).expect("valid conversion should succeed");
    let converted = std::fs::read_to_string(&destination).expect("output should be readable");
    Opm::from_xml(&converted).expect("converted output should parse");

    std::fs::write(&source, "not an OPM").expect("invalid source should be written");
    std::fs::write(&destination, b"sentinel").expect("sentinel should be restored");
    assert!(convert_file(&source, &destination, Notation::Xml).is_err());
    assert_eq!(
        std::fs::read(&destination).expect("sentinel should remain readable"),
        b"sentinel"
    );
    assert_eq!(
        std::fs::read_dir(directory.path())
            .expect("directory should be readable")
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
            .count(),
        0,
        "atomic conversion left a temporary file behind"
    );
}

#[cfg(unix)]
#[test]
fn new_destination_uses_normal_file_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let source = directory.path().join("source.opm");
    let destination = directory.path().join("destination.xml");
    std::fs::write(&source, KVN_FIXTURES[0]).expect("source should be written");
    let expected_mode = std::fs::metadata(&source).unwrap().permissions().mode() & 0o777;

    convert_file(&source, &destination, Notation::Xml).expect("valid conversion should succeed");

    let destination_mode = std::fs::metadata(destination).unwrap().permissions().mode() & 0o777;
    assert_eq!(destination_mode, expected_mode);
}

#[test]
fn file_conversion_enforces_input_limit_before_materializing_the_document() {
    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let source = directory.path().join("source.opm");
    let destination = directory.path().join("destination.xml");
    std::fs::write(&source, KVN_FIXTURES[0]).expect("source should be written");
    std::fs::write(&destination, b"sentinel").expect("sentinel should be written");

    let options = ParseOptions::default().with_max_input_bytes(16);
    let error = convert_file_with_options(&source, &destination, Notation::Xml, &options)
        .expect_err("the configured input limit should fail");

    assert_eq!(error.code(), Some("resource.input_limit_exceeded"));
    assert_eq!(
        std::fs::read(destination).expect("destination should remain readable"),
        b"sentinel"
    );
}

/// KVN has one comment slot ahead of `EPOCH`, so `data.COMMENT` and `stateVector.COMMENT` are
/// indistinguishable once written. Parsing assigns every pre-`EPOCH` comment to the data section,
/// which makes the merge canonical and idempotent rather than arbitrary.
///
/// XML keeps the two positions apart, so the model keeps both fields; only a round trip *through*
/// KVN collapses them. This test fixes that contract.
#[test]
fn kvn_merges_data_and_state_vector_comments_into_the_data_section() {
    let xml = XML_FIXTURE
        .replace("<data>", "<data>\n<COMMENT>DATA BLOCK</COMMENT>")
        .replace(
            "<stateVector>",
            "<stateVector>\n<COMMENT>STATE BLOCK</COMMENT>",
        );
    let source = Opm::from_xml(&xml).expect("XML distinguishes the two comment positions");
    assert_eq!(source.body.segment.data.comment, ["DATA BLOCK"]);
    assert_eq!(
        source.body.segment.data.state_vector.comment,
        ["STATE BLOCK"]
    );

    let kvn = convert(&xml, Notation::Kvn).expect("XML to KVN conversion should work");
    let merged = Opm::from_kvn(&kvn).expect("output KVN should parse");
    assert_eq!(
        merged.body.segment.data.comment,
        ["DATA BLOCK", "STATE BLOCK"],
        "KVN keeps both comments, in order, in the data section"
    );
    assert!(
        merged.body.segment.data.state_vector.comment.is_empty(),
        "KVN cannot address the state-vector comment position"
    );

    // The merge is idempotent: a further round trip neither loses nor duplicates a comment.
    let again = Opm::from_kvn(&merged.to_kvn().expect("merged model should generate KVN"))
        .expect("regenerated KVN should parse");
    assert_eq!(again, merged);

    // Every comment survives the trip; only its logical position is normalized.
    let round_tripped_xml = merged.to_xml().expect("merged model should generate XML");
    assert!(round_tripped_xml.contains("<COMMENT>DATA BLOCK</COMMENT>"));
    assert!(round_tripped_xml.contains("<COMMENT>STATE BLOCK</COMMENT>"));
}
