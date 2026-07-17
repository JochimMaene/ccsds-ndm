// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::generation::VersionedNdm;
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::messages::ocm::{Ocm, OcmPhysicalDescription};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::messages::tdm::{Tdm, TdmObservationData};
use ccsds_ndm::traits::{Ndm, Validate};
use ccsds_ndm::{from_str, GenerateOptions};

const OPM_KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");
const OPM_XML: &str = include_str!("../../data/xml/opm_g5.xml");
const OEM_XML: &str = include_str!("../../data/xml/oem_g14.xml");
const OCM_KVN: &str = include_str!("../../data/kvn/ocm_g15.kvn");
const OCM_MAN_KVN: &str = include_str!("../../data/kvn/ocm_g17.kvn");
const ACM_KVN: &str = include_str!("../../data/kvn/acm_g7.kvn");
const RDM_XML: &str = include_str!("../../data/xml/rdm_c4.xml");
const TDM_XML: &str = include_str!("../../data/xml/tdm_e21.xml");
const CDM_XML: &str = include_str!("../../data/xml/cdm_44.xml");
const PERMISSIVE_XML: &str = include_str!("../../data/xml/ndm_g22.xml");

#[test]
fn strict_parsing_remains_the_default() {
    assert!(from_str(PERMISSIVE_XML).is_err());
}

#[test]
fn malformed_ocm_records_are_never_silent() {
    let invalid = OCM_KVN.replacen(
        "120.0 5478.6",
        "MALFORMED TRAJECTORY RECORD\n120.0 5478.6",
        1,
    );

    assert!(Ocm::from_kvn(&invalid).is_err());
}

#[test]
fn missing_ocm_record_block_stops_fail_at_eof() {
    let trajectory_without_stop = OCM_KVN
        .strip_suffix("TRAJ_STOP\n")
        .expect("fixture ends with TRAJ_STOP");
    assert!(Ocm::from_kvn(trajectory_without_stop).is_err());

    let maneuver_without_stop = OCM_MAN_KVN
        .split_once("\nMAN_STOP")
        .expect("fixture contains MAN_STOP")
        .0;
    assert!(Ocm::from_kvn(maneuver_without_stop).is_err());
}

#[test]
fn oem_generation_rejects_non_finite_state_vectors() {
    let mut oem = Oem::from_xml(OEM_XML).unwrap();
    oem.body.segment[0].data.state_vector[0].x.value = f64::NAN;

    let error = oem.to_kvn().unwrap_err();
    assert!(error.to_string().contains("finite"));

    let mut oem = Oem::from_xml(OEM_XML).unwrap();
    oem.body.segment[0].data.covariance_matrix[0].cx_x.value = f64::INFINITY;

    let error = oem.to_kvn().unwrap_err();
    assert!(error.to_string().contains("finite"));
}

#[test]
fn opm_generation_rejects_non_finite_state_vectors() {
    let mut opm = Opm::from_xml(OPM_XML).unwrap();
    opm.body.segment.data.state_vector.x.value = f64::NAN;

    let error = opm.to_kvn().unwrap_err();
    assert!(error.to_string().contains("finite"));
}

#[test]
fn generation_rejects_nested_non_finite_values() {
    let mut tdm = Tdm::from_xml(TDM_XML).unwrap();
    tdm.body.segments[0].data.observations[0].data = TdmObservationData::Range(f64::NAN);
    assert!(tdm.to_xml().unwrap_err().to_string().contains("finite"));

    let mut cdm = Cdm::from_xml(CDM_XML).unwrap();
    cdm.body.segments[0].data.state_vector.x.value = f64::NAN;
    assert!(cdm.to_xml().unwrap_err().to_string().contains("finite"));
}

#[test]
fn rdm_generation_rejects_invalid_state_vectors() {
    let mut rdm = Rdm::from_xml(RDM_XML).unwrap();
    rdm.body.segment.data.state_vector.as_mut().unwrap().x.value = f64::NAN;

    assert!(rdm.to_xml().unwrap_err().to_string().contains("finite"));
}

#[test]
fn acm_and_ocm_collect_later_nested_validation_errors() {
    let mut acm = Acm::from_kvn(ACM_KVN).unwrap();
    acm.body.segment.data.att[0].ref_frame_a.clear();
    acm.body.segment.data.man[0].man_purpose = Some(String::new());
    assert_eq!(acm.validation_errors().unwrap().len(), 2);

    let mut ocm = Ocm::from_kvn(OCM_KVN).unwrap();
    ocm.body.segment.data.traj[0].center_name.clear();
    ocm.body.segment.data.phys = Some(OcmPhysicalDescription {
        drag_coeff_nom: Some(-1.0),
        ..OcmPhysicalDescription::default()
    });
    assert_eq!(ocm.validation_errors().unwrap().len(), 2);
}

#[test]
fn oem_generation_handles_maximum_width_records_without_panicking() {
    let mut input = OEM_XML.replacen(
        "<EPOCH>2019-12-18T12:00:00.331</EPOCH>",
        &format!("<EPOCH>{}</EPOCH>", "1".repeat(64)),
        1,
    );
    for value in [
        "2789.6", "-280.0", "-1746.8", "4.73", "-2.50", "-1.04", "0.008", "0.001", "-0.159",
    ] {
        input = input.replacen(&format!(">{value}<"), ">1.7976931348623157e308<", 1);
    }

    let oem = Oem::from_xml(&input).unwrap();
    let result = std::panic::catch_unwind(|| oem.to_kvn());
    assert!(result.is_ok(), "KVN generation panicked");
    assert!(result.unwrap().is_ok());
}

#[test]
fn generation_preserves_source_version_by_default() {
    let opm = Opm::from_kvn(OPM_KVN).unwrap();
    let output = opm.to_kvn_with(&GenerateOptions::source()).unwrap();
    assert!(output.starts_with("CCSDS_OPM_VERS"));
    assert!(output.lines().next().unwrap().ends_with("3.0"));
}

#[test]
fn unsupported_source_version_requires_explicit_upgrade() {
    let legacy = OPM_KVN.replacen("3.0", "2.0", 1);
    let opm = Opm::from_kvn(&legacy).unwrap();

    let source_error = opm.to_kvn().unwrap_err();
    assert!(source_error.to_string().contains("output version 2.0"));

    let upgraded = opm.to_kvn_with(&GenerateOptions::latest()).unwrap();
    assert!(upgraded.lines().next().unwrap().ends_with("3.0"));
}

#[test]
fn sink_writers_match_string_generation() {
    let opm = Opm::from_kvn(OPM_KVN).unwrap();
    let options = GenerateOptions::source();

    let expected_kvn = opm.to_kvn_with(&options).unwrap();
    let mut kvn = Vec::new();
    opm.write_kvn_to(&mut kvn, &options).unwrap();
    assert_eq!(kvn, expected_kvn.as_bytes());

    let expected_xml = opm.to_xml_with(&options).unwrap();
    let mut xml = Vec::new();
    opm.write_xml_to(&mut xml, &options).unwrap();
    assert_eq!(xml, expected_xml.as_bytes());
}
