use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::traits::Ndm;
use tempfile::NamedTempFile;

fn xmllint_available() -> bool {
    Command::new("xmllint").arg("--version").output().is_ok()
}

fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/xsd/ndmxml-4.0.0-master-4.0.xsd")
}

fn load_sample(rel_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../")
        .join(rel_path);
    fs::read_to_string(path).expect("failed to read sample XML")
}

fn validate_xml(xml: &str) {
    if !xmllint_available() {
        eprintln!("xmllint not available; skipping XSD validation");
        return;
    }

    let schema = schema_path();
    let tmp = NamedTempFile::new().expect("failed to create temp file");
    fs::write(tmp.path(), xml).expect("failed to write temp XML");

    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(schema.as_os_str())
        .arg(tmp.path())
        .output()
        .expect("failed to run xmllint");

    assert!(
        output.status.success(),
        "xmllint failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_xsd_schema_validation_samples() {
    let aem = Aem::from_xml(&load_sample("data/xml/aem_g11.xml")).unwrap();
    validate_xml(&aem.to_xml().unwrap());

    let apm = Apm::from_xml(&load_sample("data/xml/apm_g10.xml")).unwrap();
    validate_xml(&apm.to_xml().unwrap());

    let cdm = Cdm::from_xml(&load_sample("data/xml/cdm_44.xml")).unwrap();
    validate_xml(&cdm.to_xml().unwrap());

    let ocm = Ocm::from_xml(&load_sample("data/xml/ocm_g20.xml")).unwrap();
    validate_xml(&ocm.to_xml().unwrap());

    let oem = Oem::from_xml(&load_sample("data/xml/oem_g14.xml")).unwrap();
    validate_xml(&oem.to_xml().unwrap());

    let omm = Omm::from_xml(&load_sample("data/xml/omm_g10.xml")).unwrap();
    validate_xml(&omm.to_xml().unwrap());

    let opm = Opm::from_xml(&load_sample("data/xml/opm_g5.xml")).unwrap();
    validate_xml(&opm.to_xml().unwrap());

    let rdm = Rdm::from_xml(&load_sample("data/xml/rdm_c3.xml")).unwrap();
    validate_xml(&rdm.to_xml().unwrap());

    let tdm = Tdm::from_xml(&load_sample("data/xml/tdm_e21.xml")).unwrap();
    validate_xml(&tdm.to_xml().unwrap());

    let ndm = CombinedNdm::from_xml(&load_sample("data/xml/ndm_g12.xml")).unwrap();
    validate_xml(&ndm.to_xml().unwrap());
}
