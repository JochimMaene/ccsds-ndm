use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{convert, convert_file, convert_file_with_options, Notation, ParseOptions};

const KVN_FIXTURES: [&str; 3] = [
    include_str!("../data/kvn/oem_g11.kvn"),
    include_str!("../data/kvn/oem_g12.kvn"),
    include_str!("../data/kvn/oem_g13.kvn"),
];
const XML: &str = include_str!("../data/xml/oem_g14.xml");

#[test]
fn both_directions_preserve_the_complete_typed_model() {
    for source in KVN_FIXTURES {
        let expected = Oem::from_kvn(source).unwrap();
        let xml = convert(source, Notation::Xml).unwrap();
        assert_eq!(Oem::from_xml(&xml).unwrap(), expected);
    }

    let expected = Oem::from_xml(XML).unwrap();
    let kvn = convert(XML, Notation::Kvn).unwrap();
    assert_eq!(Oem::from_kvn(&kvn).unwrap(), expected);
}

#[test]
fn xml_to_kvn_rejects_partial_acceleration_without_emitting_ambiguous_data() {
    let mut message = Oem::from_xml(XML).unwrap();
    let state = &mut message.body.segment[0].data.state_vector[0];
    state.y_ddot = None;
    state.z_ddot = None;
    let xml = message
        .to_xml()
        .expect("partial acceleration is representable in XML");

    let error = convert(&xml, Notation::Kvn)
        .expect_err("partial acceleration is not representable in OEM KVN");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[0].data.state_vector[0]")
    );
}

#[test]
fn xml_to_kvn_rejects_comments_that_cannot_keep_their_covariance_association() {
    let mut message = Oem::from_xml(XML).unwrap();
    let mut second = message.body.segment[0].data.covariance_matrix[0].clone();
    second.epoch = "2019-12-28T23:28:00.331".parse().unwrap();
    second.comment = vec!["belongs to the second covariance".into()];
    message.body.segment[0].data.covariance_matrix.push(second);
    let xml = message.to_xml().unwrap();

    let error = convert(&xml, Notation::Kvn)
        .expect_err("KVN cannot preserve a later covariance's comment association");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[0].data.covariance_matrix[1].comment")
    );
}

#[test]
fn xml_to_kvn_round_trip_preserves_empty_comments() {
    let mut message = Oem::from_xml(XML).unwrap();
    message.header.comment = vec![String::new()];

    let kvn = convert(&message.to_xml().unwrap(), Notation::Kvn).unwrap();
    assert!(kvn.lines().any(|line| line == "COMMENT "));
    let reparsed = Oem::from_kvn(&kvn).unwrap();
    assert_eq!(reparsed.header.comment, vec![""]);
    assert_eq!(
        Oem::from_xml(&reparsed.to_xml().unwrap()).unwrap(),
        reparsed
    );
}

#[test]
fn kvn_round_trip_preserves_significant_comment_whitespace() {
    let mut message = Oem::from_xml(XML).unwrap();
    message.header.comment = vec!["   indented   ".into()];

    let kvn = convert(&message.to_xml().unwrap(), Notation::Kvn).unwrap();
    let reparsed = Oem::from_kvn(&kvn).unwrap();
    assert_eq!(reparsed.header.comment, message.header.comment);
}

#[test]
fn file_conversion_is_atomic_and_bounds_input_before_materialization() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.oem");
    let destination = directory.path().join("destination.xml");
    std::fs::write(&source, KVN_FIXTURES[0]).unwrap();
    std::fs::write(&destination, b"sentinel").unwrap();

    convert_file(&source, &destination, Notation::Xml).unwrap();
    Oem::from_xml(&std::fs::read_to_string(&destination).unwrap()).unwrap();

    std::fs::write(&destination, b"sentinel").unwrap();
    let error = convert_file_with_options(
        &source,
        &destination,
        Notation::Xml,
        &ParseOptions::default().with_max_input_bytes(16),
    )
    .expect_err("input limit should fail before replacement");
    assert_eq!(error.code(), Some("resource.input_limit_exceeded"));
    assert_eq!(std::fs::read(&destination).unwrap(), b"sentinel");
    assert_eq!(
        std::fs::read_dir(directory.path())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
            .count(),
        0
    );
}
