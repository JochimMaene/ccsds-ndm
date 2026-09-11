use crate::common::{data_dir, validate_xml};
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::Ndm;
use std::fs;

/// Every `aem_*` fixture in `data/<extension>/`, sorted for a stable failure order.
///
/// Discovered rather than listed so a newly shipped fixture is covered without editing a test.
fn fixtures(extension: &str) -> Vec<(String, String)> {
    let directory = data_dir().join(extension);
    let mut found: Vec<(String, String)> = fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("{} is not readable: {error}", directory.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension().is_some_and(|value| value == extension)
                && path
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with("aem_"))
        })
        .map(|path| {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            (name, fs::read_to_string(&path).unwrap())
        })
        .collect();
    found.sort();
    assert!(!found.is_empty(), "no aem_* {extension} fixtures found");
    found
}

#[test]
fn every_shipped_aem_fixture_preserves_states_and_generates_valid_xml() {
    for (name, source) in fixtures("kvn") {
        let message = Aem::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Aem::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Aem::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(&name, &xml);
    }
    for (name, source) in fixtures("xml") {
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
    }
}
