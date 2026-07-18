use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{convert_oem, convert_oem_file, GenerateOptions, Notation, ParseOptions};

const KVN_FIXTURES: [&str; 3] = [
    include_str!("../../data/kvn/oem_g11.kvn"),
    include_str!("../../data/kvn/oem_g12.kvn"),
    include_str!("../../data/kvn/oem_g13.kvn"),
];
const XML: &str = include_str!("../../data/xml/oem_g14.xml");

#[test]
fn both_directions_preserve_the_complete_typed_model() {
    for source in KVN_FIXTURES {
        let expected = Oem::from_kvn(source).unwrap();
        let xml = convert_oem(
            source,
            Notation::Kvn,
            Notation::Xml,
            &ParseOptions::default(),
            &GenerateOptions::source(),
        )
        .unwrap();
        assert_eq!(Oem::from_xml(&xml).unwrap(), expected);
    }

    let expected = Oem::from_xml(XML).unwrap();
    let kvn = convert_oem(
        XML,
        Notation::Xml,
        Notation::Kvn,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
    .unwrap();
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

    let error = convert_oem(
        &xml,
        Notation::Xml,
        Notation::Kvn,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
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

    let error = convert_oem(
        &xml,
        Notation::Xml,
        Notation::Kvn,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
    .expect_err("KVN cannot preserve a later covariance's comment association");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[0].data.covariance_matrix[1].comment")
    );
}

#[test]
fn file_conversion_is_atomic_and_bounds_input_before_materialization() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.oem");
    let destination = directory.path().join("destination.xml");
    std::fs::write(&source, KVN_FIXTURES[0]).unwrap();
    std::fs::write(&destination, b"sentinel").unwrap();

    convert_oem_file(
        &source,
        &destination,
        Notation::Kvn,
        Notation::Xml,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
    .unwrap();
    Oem::from_xml(&std::fs::read_to_string(&destination).unwrap()).unwrap();

    std::fs::write(&destination, b"sentinel").unwrap();
    let error = convert_oem_file(
        &source,
        &destination,
        Notation::Kvn,
        Notation::Xml,
        &ParseOptions::default().with_max_input_bytes(16),
        &GenerateOptions::source(),
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
