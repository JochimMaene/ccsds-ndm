// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Regressions for the pre-release conformance review.
use ccsds_ndm::messages::{
    acm::Acm, cdm::Cdm, ndm::CombinedNdm, ocm::Ocm, omm::Omm, opm::Opm, tdm::Tdm,
};
use ccsds_ndm::{Message, Ndm};

#[test]
fn gm_values_reject_unit_only_or_incompatible_annotations() {
    let source = include_str!("../data/kvn/opm_g2.kvn");
    for value in ["[km**3/s**2]", "[s]", "[garbage]", "398600.4415 [s]"] {
        let invalid = source.replace("GM = 398600.4415 [km**3/s**2]", &format!("GM = {value}"));
        let error = Opm::from_kvn(&invalid).unwrap_err();
        assert!(error.to_string().contains("GM"), "{error}");
    }
}

#[test]
fn acm_metadata_classification_is_preserved_in_kvn_and_refused_in_xml() {
    let source = include_str!("../data/kvn/acm_g6.kvn")
        .replace("META_START", "META_START\nCLASSIFICATION = PUBLIC");
    let message = Acm::from_kvn(&source).unwrap();
    assert_eq!(
        message.body.segment.metadata.classification.as_deref(),
        Some("PUBLIC")
    );
    assert_eq!(Acm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    let mut sink = Vec::new();
    assert!(message.write_xml_to(&mut sink).is_err());
    assert!(sink.is_empty());
}

#[test]
fn empty_user_blocks_are_read_but_never_generated() {
    for fixture in [
        include_str!("../data/kvn/acm_g6.kvn"),
        include_str!("../data/kvn/ocm_g15.kvn"),
    ] {
        for content in ["", "COMMENT no parameters\n"] {
            let source = format!("{fixture}\nUSER_START\n{content}USER_STOP\n");
            let message = ccsds_ndm::from_str(&source).unwrap();
            assert!(message.to_kvn().is_err());
            assert!(message.to_xml().is_err());
        }
    }
}

#[test]
fn sgp4_semantics_apply_before_every_generation_route() {
    let original = Omm::from_kvn(include_str!("../data/kvn/omm_g9.kvn")).unwrap();
    for field in ["CENTER_NAME", "REF_FRAME", "TIME_SYSTEM"] {
        let mut invalid = original.clone();
        let metadata = &mut invalid.body.segment.metadata;
        match field {
            "CENTER_NAME" => metadata.center_name = "MARS".into(),
            "REF_FRAME" => metadata.ref_frame = "GCRF".into(),
            _ => metadata.time_system = "TAI".into(),
        }
        assert!(invalid.validate().unwrap_err().to_string().contains(field));
        let mut sink = Vec::new();
        assert!(invalid.write_xml_to(&mut sink).is_err());
        assert!(sink.is_empty());
    }
}

#[test]
fn tdm_xml_does_not_accept_kvn_category_padding_or_blank_aliases() {
    let source = include_str!("../data/kvn/tdm_e1.kvn");
    let mut original = Tdm::from_kvn(source).unwrap();
    original.body.segments[0].metadata.angle_type = Some("RADEC".parse().unwrap());
    original.body.segments[0].metadata.reference_frame = Some("EME2000".parse().unwrap());
    let xml = original.to_xml().unwrap();
    for (before, after) in [
        ("<ANGLE_TYPE>RADEC<", "<ANGLE_TYPE> RADEC <"),
        ("<REFERENCE_FRAME>EME2000<", "<REFERENCE_FRAME>EME 2000<"),
    ] {
        assert!(Tdm::from_xml(&xml.replace(before, after)).is_err());
    }
}

#[test]
fn controlled_reentry_errors_name_the_containing_assignment() {
    let source = include_str!("../data/kvn/rdm_c1.kvn");
    let invalid = source.replace("CONTROLLED_REENTRY = NO", "CONTROLLED_REENTRY = SOMETIMES");
    assert_ne!(source, invalid);
    assert!(ccsds_ndm::from_str(&invalid)
        .unwrap_err()
        .to_string()
        .contains("CONTROLLED_REENTRY"));
}

#[test]
fn cdm_line_rules_do_not_leak_into_other_combined_constituents() {
    let cdm = Cdm::from_kvn(include_str!("../data/kvn/cdm_362.kvn")).unwrap();
    let mut ocm = Ocm::from_kvn(include_str!("../data/kvn/ocm_g15.kvn")).unwrap();
    ocm.header.comment = vec![format!("Café {}", "x".repeat(300))];
    let combined = CombinedNdm {
        id: None,
        comments: vec![],
        messages: vec![Message::Cdm(cdm), Message::Ocm(ocm)],
    };
    let xml = combined.to_xml().unwrap();
    assert!(xml.contains("Café"));
    assert_eq!(CombinedNdm::from_xml(&xml).unwrap(), combined);
    let mut sink = Vec::new();
    combined.write_xml_to(&mut sink).unwrap();
    assert_eq!(sink, xml.as_bytes());
}
