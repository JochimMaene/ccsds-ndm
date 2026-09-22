use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;
#[test]
fn display_includes_structured_operation_context() {
    let parse_error = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\nBROKEN").unwrap_err();
    let parse_display = parse_error.to_string();
    assert!(parse_display.contains("failed to parse OPM KVN 3.0"));

    let mut message = Opm::from_kvn(include_str!("../../data/kvn/opm_g1.kvn")).unwrap();
    message.body.segment.metadata.object_name.clear();
    let generation_display = message.to_xml().unwrap_err().to_string();
    assert!(generation_display.contains("failed to generate OPM XML 3.0"));
}
