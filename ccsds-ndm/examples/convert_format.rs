use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

fn main() {
    let kvn_input = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = EXAMPLE
OBJECT_NAME = SATELLITE
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T12:00:00
X = 6500.0 [km]
Y = 0.0 [km]
Z = 0.0 [km]
X_DOT = 0.0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 0.0 [km/s]
"#;

    println!("Input KVN:\n{}", kvn_input);

    // 1. Parse KVN
    let opm = Opm::from_kvn(kvn_input).expect("Failed to parse KVN");

    // 2. Convert to XML
    let xml_output = opm.to_xml().expect("Failed to convert to XML");

    println!("\nConverted XML:\n{}", xml_output);

    // 3. Verify round-trip (XML back to Opm)
    let opm_from_xml = Opm::from_xml(&xml_output).expect("Failed to parse generated XML");

    assert_eq!(
        opm.body.segment.metadata.object_name,
        opm_from_xml.body.segment.metadata.object_name
    );
    println!("\nRound-trip verification successful!");
}
