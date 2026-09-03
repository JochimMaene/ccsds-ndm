use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

fn xml() -> String {
    Opm::from_kvn(include_str!("../data/kvn/opm_g1.kvn"))
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML")
}

fn assert_rejected(label: &str, xml: String) {
    assert!(
        Opm::from_xml(&xml).is_err(),
        "{label} was silently accepted"
    );
}

#[test]
fn strict_xml_rejects_wrong_root_unknown_content_duplicates_and_trailing_documents() {
    let source = xml();
    assert_rejected(
        "wrong root",
        source.replace("<opm ", "<omm ").replace("</opm>", "</omm>"),
    );
    assert_rejected(
        "unknown root attribute",
        source.replace("<opm ", "<opm unexpected=\"value\" "),
    );
    assert_rejected(
        "unknown metadata element",
        source.replace("<metadata>", "<metadata><UNKNOWN>value</UNKNOWN>"),
    );
    assert_rejected(
        "unknown metadata attribute",
        source.replace("<metadata>", "<metadata unexpected=\"value\">"),
    );

    let object_name = "<OBJECT_NAME>OSPREY 5</OBJECT_NAME>";
    assert!(source.contains(object_name));
    assert_rejected(
        "duplicate fixed element",
        source.replace(object_name, &format!("{object_name}{object_name}")),
    );
    let object_id = "<OBJECT_ID>1998-999A</OBJECT_ID>";
    assert!(source.contains(&format!("{object_name}{object_id}")));
    assert_rejected(
        "reordered metadata elements",
        source.replace(
            &format!("{object_name}{object_id}"),
            &format!("{object_id}{object_name}"),
        ),
    );
    assert_rejected("trailing element", format!("{source}<junk/>"));
    assert_rejected("multiple documents", format!("{source}{source}"));
    assert_rejected(
        "document type declaration",
        source.replacen("<opm ", "<!DOCTYPE opm><opm ", 1),
    );
}

#[test]
fn strict_xml_rejects_attributes_the_schema_does_not_declare() {
    let source = Opm::from_kvn(include_str!("../data/kvn/opm_g2.kvn"))
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML");

    for (label, from, to) in [
        (
            "unknown attribute on a measure element",
            "<X units=\"km\">",
            "<X units=\"km\" unexpected=\"value\">",
        ),
        (
            "unknown attribute on a text element",
            "<OBJECT_NAME>",
            "<OBJECT_NAME unexpected=\"value\">",
        ),
        (
            "units on a unitless element",
            "<ECCENTRICITY>",
            "<ECCENTRICITY units=\"km\">",
        ),
        (
            "units on an epoch element",
            "<EPOCH>",
            "<EPOCH units=\"s\">",
        ),
        (
            "nil on a mandatory element",
            "<X units=\"km\">",
            "<X units=\"km\" nil=\"true\">",
        ),
        (
            "parameter attribute outside USER_DEFINED",
            "<MASS units=\"kg\">",
            "<MASS units=\"kg\" parameter=\"FOO\">",
        ),
        (
            "invalid units hidden by nil",
            "<MASS units=\"kg\">",
            "<MASS units=\"km\" nil=\"true\">",
        ),
    ] {
        assert!(source.contains(from), "fixture should contain {from}");
        assert_rejected(label, source.replace(from, to));
    }
}

#[test]
fn strict_xml_keeps_schema_attributes_and_the_documented_nil_extension() {
    let source = Opm::from_kvn(include_str!("../data/kvn/opm_g2.kvn"))
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML");

    // `units` is schema-defined; `nil` on an otherwise attribute-free optional value is a
    // documented compatibility extension.
    Opm::from_xml(&source).expect("generated XML should round-trip");
    let with_nil = source.replace("<DRAG_COEFF>2.3</DRAG_COEFF>", "<DRAG_COEFF nil=\"true\"/>");
    assert_ne!(with_nil, source, "fixture should contain DRAG_COEFF");
    let parsed = Opm::from_xml(&with_nil).expect("nil-marked optional value should be accepted");
    assert!(
        parsed
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .is_some_and(|parameters| parameters.drag_coeff.is_none()),
        "nil DRAG_COEFF should deserialize as absent"
    );
}

#[test]
fn strict_xml_accepts_namespace_metadata_and_does_not_confuse_processing_instructions() {
    let source = xml();
    let source = source.replacen(
        "?>",
        "?><?xml-stylesheet type=\"text/xsl\" href=\"opm.xsl\"?>",
        1,
    );
    let source = source.replacen(
        "<opm ",
        "<opm xmlns:ndm=\"urn:ccsds:ndm\" xsi:schemaLocation=\"urn:ccsds:ndm opm.xsd\" ",
        1,
    );
    Opm::from_xml(&source).expect("namespace metadata and xml-stylesheet PI should parse");

    assert_rejected(
        "default namespace that qualifies the OPM root",
        xml().replacen("<opm ", "<opm xmlns=\"urn:ccsds:ndm\" ", 1),
    );
}
