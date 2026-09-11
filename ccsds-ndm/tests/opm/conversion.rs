use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;
use ccsds_ndm::{convert, Notation};

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
