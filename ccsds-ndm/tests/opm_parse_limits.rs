use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;
use ccsds_ndm::{from_str_with_options, ParseOptions};

const KVN: &str = include_str!("../data/kvn/opm_g1.kvn");

fn xml() -> String {
    Opm::from_kvn(KVN)
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML")
}

#[test]
fn aggregate_input_limit_is_optional_and_exact_for_both_notations() {
    let xml = xml();
    for (label, input) in [("KVN", KVN), ("XML", xml.as_str())] {
        from_str_with_options(
            input,
            None,
            &ParseOptions::default().with_max_input_bytes(input.len()),
        )
        .unwrap_or_else(|error| panic!("exact {label} input limit failed: {error}"));

        let error = from_str_with_options(
            input,
            None,
            &ParseOptions::default().with_max_input_bytes(input.len() - 1),
        )
        .expect_err("one-byte-small input limit should fail");
        assert_eq!(error.code(), Some("resource.input_limit_exceeded"));
        assert_eq!(error.field_path(), None);
    }
}

#[test]
fn xml_depth_has_a_safe_default_and_a_caller_override() {
    let xml = xml();
    Opm::from_xml(&xml).expect("schema-valid OPM should fit the default depth");

    let error = from_str_with_options(&xml, None, &ParseOptions::default().with_max_xml_depth(1))
        .expect_err("depth one should reject a nested OPM");
    assert_eq!(error.code(), Some("resource.xml_depth_limit_exceeded"));

    from_str_with_options(&xml, None, &ParseOptions::default().with_max_xml_depth(32))
        .expect("caller should be able to select a larger depth");
}
