use crate::{KVN_FIXTURES, XML};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::{convert, Ndm, Notation, Validate};

#[test]
fn both_directions_reproduce_the_generated_text() {
    for source in KVN_FIXTURES {
        let kvn = Oem::from_kvn(source).unwrap().to_kvn().unwrap();
        let xml = convert(source, Notation::Xml).unwrap();
        assert_eq!(convert(&xml, Notation::Kvn).unwrap(), kvn);
    }
    let xml = Oem::from_xml(XML).unwrap().to_xml().unwrap();
    let kvn = convert(XML, Notation::Kvn).unwrap();
    assert_eq!(convert(&kvn, Notation::Xml).unwrap(), xml);
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
    message.body.segment[0].metadata.stop_time = second.epoch;
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
fn kvn_round_trip_keeps_leading_comment_blanks_only() {
    // Leading blanks are text; trailing blanks before the end of line are not
    // significant in KVN (ODM 7.4.7).
    let mut message = Oem::from_xml(XML).unwrap();
    message.header.comment = vec!["   indented   ".into()];

    let kvn = convert(&message.to_xml().unwrap(), Notation::Kvn).unwrap();
    let reparsed = Oem::from_kvn(&kvn).unwrap();
    assert_eq!(reparsed.header.comment, vec!["   indented"]);
}

#[test]
fn xml_whitespace_around_text_values_is_padding_in_both_notations() {
    // XML keeps a padded xsd:string as written (8.13.5) and KVN drops padding (7.4.7). The
    // padded value names the same object and time system, and KVN output drops the padding
    // instead of failing on a line break it cannot hold.
    let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    let object_name = message.body.segment[0].metadata.object_name.clone();
    let metadata = &mut message.body.segment[1].metadata;
    metadata.object_name = format!("\n  {object_name}\n\t");
    metadata.time_system = "\r\nutc\n".into();
    metadata.center_name = "\tEARTH ".into();
    message.validate().expect("padding names the same object");

    let xml = message.to_xml().unwrap();
    assert_eq!(
        Oem::from_xml(&xml).unwrap(),
        message,
        "XML keeps the padding"
    );
    let kvn = convert(&xml, Notation::Kvn).unwrap();
    let metadata = &Oem::from_kvn(&kvn).unwrap().body.segment[1].metadata;
    assert_eq!(metadata.object_name, object_name);
    assert_eq!(metadata.time_system, "utc");
    assert_eq!(metadata.center_name, "EARTH");

    // A line break inside the value is not padding.
    message.body.segment[1].metadata.center_name = "EA\nRTH".into();
    assert_eq!(
        message.to_kvn().unwrap_err().field_path().as_deref(),
        Some("body.segment[1].metadata.center_name")
    );
}
