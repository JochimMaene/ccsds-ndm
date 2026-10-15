use crate::common::mutated;
use crate::common::{fixtures, validate_xml};

use ccsds_ndm::{from_str, Message};

// OPM, OEM, and AEM keep their corpus checks in their family suites.
const FAMILY_PREFIXES: [&str; 7] = ["omm_", "ocm_", "cdm_", "tdm_", "rdm_", "apm_", "acm_"];

fn assert_kvn_generation(label: &str, message: &Message, expected: &Message) {
    let kvn = message
        .to_kvn()
        .unwrap_or_else(|error| panic!("{label} KVN generation failed: {error}"));
    assert_eq!(message.to_kvn().unwrap(), kvn, "{label} KVN changed");
    let reparsed_kvn = from_str(&kvn)
        .unwrap_or_else(|error| panic!("{label} generated KVN did not parse: {error}"));
    assert_eq!(&reparsed_kvn, expected, "{label} KVN model");
}

fn assert_xml_generation(label: &str, message: &Message) {
    let xml = message
        .to_xml()
        .unwrap_or_else(|error| panic!("{label} XML generation failed: {error}"));
    assert_eq!(message.to_xml().unwrap(), xml, "{label} XML changed");
    validate_xml(label, &xml);
    let reparsed_xml = from_str(&xml)
        .unwrap_or_else(|error| panic!("{label} generated XML did not parse: {error}"));
    assert_eq!(&reparsed_xml, message, "{label} XML model");
}

#[test]
fn kvn_fixtures_preserve_models_and_generate_deterministically() {
    for prefix in FAMILY_PREFIXES {
        for (label, input) in fixtures(prefix, "kvn") {
            let message = from_str(&input)
                .unwrap_or_else(|error| panic!("{label} strict parse failed: {error}"));
            assert_kvn_generation(&label, &message, &message);
            assert_xml_generation(&label, &message);
        }
    }
}

#[test]
fn xml_fixtures_preserve_models_and_generate_deterministically() {
    for (label, input) in fixtures("", "xml").into_iter().filter(|(name, _)| {
        FAMILY_PREFIXES
            .iter()
            .any(|prefix| name.starts_with(prefix))
    }) {
        let message =
            from_str(&input).unwrap_or_else(|error| panic!("{label} strict parse failed: {error}"));
        assert_xml_generation(&label, &message);
        match label.as_ref() {
            // KVN cannot represent Unicode participant names, nested CDM comment
            // associations, or multiline OCM assignment/history values.
            "tdm_e21.xml" | "cdm_44.xml" | "ocm_g20.xml" => {
                let error = message.to_kvn().expect_err(&label);
                // CDM and OCM use generic validation errors without stable codes.
                let reason = if label == "cdm_44.xml" {
                    "COMMENT"
                } else {
                    "printable ASCII"
                };
                assert!(error.to_string().contains(reason), "{label}: {error}");
            }
            "apm_g10.xml" => {
                // A multiline XML comment becomes separate KVN COMMENT records.
                let normalized = mutated(
                    &input,
                    "launch\ntime</COMMENT>",
                    "launch</COMMENT><COMMENT>time</COMMENT>",
                );
                assert_ne!(normalized, input, "{label} multiline comment is missing");
                assert_kvn_generation(&label, &message, &from_str(&normalized).unwrap());
            }
            "omm_g10.xml" => {
                // KVN assignment values omit surrounding whitespace, and trailing comment
                // blanks are not significant (ODM 7.4.7).
                let normalized = mutated(&input, "<MESSAGE_ID> OMM", "<MESSAGE_ID>OMM");
                let normalized = mutated(&normalized, "OMM </COMMENT>", "OMM</COMMENT>");
                assert_ne!(normalized, input, "{label} padded message ID is missing");
                assert_kvn_generation(&label, &message, &from_str(&normalized).unwrap());
            }
            _ => assert_kvn_generation(&label, &message, &message),
        }
    }
}
