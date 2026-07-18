use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{convert_opm, convert_opm_file, GenerateOptions, Notation, ParseOptions};

const KVN_FIXTURES: [&str; 4] = [
    include_str!("../../data/kvn/opm_g1.kvn"),
    include_str!("../../data/kvn/opm_g2.kvn"),
    include_str!("../../data/kvn/opm_g3.kvn"),
    include_str!("../../data/kvn/opm_g4.kvn"),
];
const XML_FIXTURE: &str = include_str!("../../data/xml/opm_g5.xml");

#[test]
fn both_conversion_directions_preserve_the_complete_typed_model() {
    for source in KVN_FIXTURES {
        let expected = Opm::from_kvn(source).expect("KVN fixture should parse");
        let xml = convert_opm(
            source,
            Notation::Kvn,
            Notation::Xml,
            &ParseOptions::default(),
            &GenerateOptions::source(),
        )
        .expect("KVN to XML conversion should work");
        assert_eq!(
            Opm::from_xml(&xml).expect("output XML should parse"),
            expected
        );
    }

    let expected = Opm::from_xml(XML_FIXTURE).expect("XML fixture should parse");
    let kvn = convert_opm(
        XML_FIXTURE,
        Notation::Xml,
        Notation::Kvn,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
    .expect("XML to KVN conversion should work");
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

    let kvn = convert_opm(
        &xml,
        Notation::Xml,
        Notation::Kvn,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
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

    convert_opm_file(
        &source,
        &destination,
        Notation::Kvn,
        Notation::Xml,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
    .expect("valid conversion should succeed");
    let converted = std::fs::read_to_string(&destination).expect("output should be readable");
    Opm::from_xml(&converted).expect("converted output should parse");

    std::fs::write(&source, "not an OPM").expect("invalid source should be written");
    std::fs::write(&destination, b"sentinel").expect("sentinel should be restored");
    assert!(convert_opm_file(
        &source,
        &destination,
        Notation::Kvn,
        Notation::Xml,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
    .is_err());
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

#[test]
fn file_conversion_enforces_input_limit_before_materializing_the_document() {
    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let source = directory.path().join("source.opm");
    let destination = directory.path().join("destination.xml");
    std::fs::write(&source, KVN_FIXTURES[0]).expect("source should be written");
    std::fs::write(&destination, b"sentinel").expect("sentinel should be written");

    let options = ParseOptions::default().with_max_input_bytes(16);
    let error = convert_opm_file(
        &source,
        &destination,
        Notation::Kvn,
        Notation::Xml,
        &options,
        &GenerateOptions::source(),
    )
    .expect_err("the configured input limit should fail");

    assert_eq!(error.code(), Some("resource.input_limit_exceeded"));
    assert_eq!(
        std::fs::read(destination).expect("destination should remain readable"),
        b"sentinel"
    );
}
