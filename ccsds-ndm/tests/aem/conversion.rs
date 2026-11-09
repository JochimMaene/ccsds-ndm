use crate::common::{fixtures, validate_xml};
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::Ndm;

#[test]
fn every_shipped_aem_fixture_preserves_states_and_generates_valid_xml() {
    for (name, source) in fixtures("aem_", "kvn") {
        let message = Aem::from_kvn(&source).unwrap();
        if name == "aem_g4.kvn" {
            assert_eq!(message.body.segment.len(), 2);
        }
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Aem::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Aem::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(&name, &xml);
    }
    for (name, source) in fixtures("aem_", "xml") {
        let message = Aem::from_xml(&source).unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Aem::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(&name, &xml);
        let kvn = message.to_kvn().unwrap();
        let normalized = Aem::from_kvn(&kvn).unwrap();
        // ADM 6.9.2 forbids KVN history units; 7.6.10 makes fixed XML units optional.
        // These fixtures need only unit omission: all numeric values fit KVN precision.
        let expected_xml = source
            .replace(" units=\"deg\"", "")
            .replace(" units=\"deg/s\"", "")
            .replace(" units=\"s\"", "");
        let expected = Aem::from_xml(&expected_xml).unwrap();
        assert_eq!(normalized, expected, "{name} normalized KVN model");
        if name == "aem_g13.xml" {
            assert!(source.contains("<NUTATION units=\"deg\">"));
            assert!(!kvn.contains("[deg]"));
            assert!(!kvn.contains("[deg/s]"));
            let normalized_xml = normalized.to_xml().unwrap();
            assert!(normalized_xml.contains("<NUTATION>2</NUTATION>"));
            assert!(!normalized_xml.contains("<NUTATION units="));
            validate_xml("AEM optional unit normalization", &normalized_xml);
        }
    }
}
