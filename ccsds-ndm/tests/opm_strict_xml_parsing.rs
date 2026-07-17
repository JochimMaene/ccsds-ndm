use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

fn xml() -> String {
    Opm::from_kvn(include_str!("../../data/kvn/opm_g1.kvn"))
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
