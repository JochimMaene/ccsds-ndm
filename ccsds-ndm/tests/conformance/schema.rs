use crate::common::mutated;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/xsd/ndmxml-4.0.0-master-4.0.xsd")
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
    let control = include_str!("../../data/xml/rdm_c3.xml");
    let valid = xerces_validates(control);
    assert!(
        valid.status.success(),
        "Xerces rejected the control: {}",
        String::from_utf8_lossy(&valid.stderr)
    );

    let nan = mutated(
        control,
        "<ORBIT_LIFETIME units=\"d\">23.0</ORBIT_LIFETIME>",
        "<ORBIT_LIFETIME units=\"d\">NaN</ORBIT_LIFETIME>",
    );
    let invalid = xerces_validates(&nan);
    assert!(
        !invalid.status.success(),
        "Xerces accepted NaN for positiveDouble"
    );
}
