use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::Ndm;
mod common;
use common::validate_xml;

fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/xsd/ndmxml-4.0.0-master-4.0.xsd")
}

fn load_sample(rel_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path);
    fs::read_to_string(path).expect("failed to read sample XML")
}

fn xerces_validates(xml: &str) -> std::process::Output {
    const VALIDATOR: &str = r#"
import java.io.File;
import javax.xml.XMLConstants;
import javax.xml.transform.stream.StreamSource;
import javax.xml.validation.SchemaFactory;

class XsdValidate {
    public static void main(String[] args) throws Exception {
        var factory = SchemaFactory.newInstance(XMLConstants.W3C_XML_SCHEMA_NS_URI);
        var schema = factory.newSchema(new File(args[0]));
        schema.newValidator().validate(new StreamSource(new File(args[1])));
    }
}
"#;

    let directory = tempfile::tempdir().expect("failed to create Xerces test directory");
    let source = directory.path().join("XsdValidate.java");
    let document = directory.path().join("document.xml");
    fs::write(&source, VALIDATOR).expect("failed to write Xerces validator");
    fs::write(&document, xml).expect("failed to write Xerces document");
    Command::new("java")
        .arg(source)
        .arg(schema_path())
        .arg(document)
        .output()
        .expect("Java/Xerces is required for XSD facet oracle tests")
}

#[test]
fn xerces_oracle_rejects_nan_for_positive_double() {
    let control = load_sample("data/xml/rdm_c3.xml");
    let valid = xerces_validates(&control);
    assert!(
        valid.status.success(),
        "Xerces rejected the control: {}",
        String::from_utf8_lossy(&valid.stderr)
    );

    let nan = control.replace(
        "<ORBIT_LIFETIME units=\"d\">23.0</ORBIT_LIFETIME>",
        "<ORBIT_LIFETIME units=\"d\">NaN</ORBIT_LIFETIME>",
    );
    let invalid = xerces_validates(&nan);
    assert!(
        !invalid.status.success(),
        "Xerces accepted NaN for positiveDouble"
    );
}

#[test]
fn test_xsd_schema_validation_samples() {
    // AEM, OEM, and OPM are absent on purpose: their own suites already sweep every
    // shipped fixture through this schema (tests/aem/conversion.rs,
    // tests/oem/generation.rs, tests/opm/generation.rs).
    let apm = Apm::from_xml(&load_sample("data/xml/apm_g10.xml")).unwrap();
    validate_xml("apm_g10.xml", &apm.to_xml().unwrap());

    let cdm = Cdm::from_xml(&load_sample("data/xml/cdm_44.xml")).unwrap();
    validate_xml("cdm_44.xml", &cdm.to_xml().unwrap());

    let ocm = Ocm::from_xml(&load_sample("data/xml/ocm_g20.xml")).unwrap();
    validate_xml("ocm_g20.xml", &ocm.to_xml().unwrap());

    let omm = Omm::from_xml(&load_sample("data/xml/omm_g10.xml")).unwrap();
    validate_xml("omm_g10.xml", &omm.to_xml().unwrap());

    let rdm = Rdm::from_xml(&load_sample("data/xml/rdm_c3.xml")).unwrap();
    validate_xml("rdm_c3.xml", &rdm.to_xml().unwrap());

    let tdm = Tdm::from_xml(&load_sample("data/xml/tdm_e21.xml")).unwrap();
    validate_xml("tdm_e21.xml", &tdm.to_xml().unwrap());

    let ndm = CombinedNdm::from_xml(&load_sample("data/xml/ndm_g12.xml")).unwrap();
    validate_xml("ndm_g12.xml", &ndm.to_xml().unwrap());
}
