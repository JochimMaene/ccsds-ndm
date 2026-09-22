// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::mutated;

use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::Ndm;

#[test]
fn xml_reference_frame_epoch_requires_calendar_form() {
    let source = include_str!("../../data/xml/omm_g10.xml");
    let valid = mutated(
        source,
        "<REF_FRAME>TEME</REF_FRAME>",
        "<REF_FRAME>TEME</REF_FRAME>\n<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>",
    );

    let omm = Omm::from_xml(&valid).expect("calendar frame epoch should parse");
    assert_eq!(
        omm.body
            .segment
            .metadata
            .ref_frame_epoch
            .as_ref()
            .unwrap()
            .as_str(),
        "2000-01-01T12:00:00"
    );

    let generated = omm.to_xml().expect("valid OMM should generate");
    assert!(generated.contains("<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>"));

    let numeric = mutated(
        &valid,
        "<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>",
        "<REF_FRAME_EPOCH>123.5</REF_FRAME_EPOCH>",
    );
    crate::common::assert_invalid_epoch(&Omm::from_xml(&numeric).unwrap_err(), "123.5");

    let numeric_mean_epoch = mutated(
        &valid,
        "<EPOCH>2020-064T10:34:41.4264</EPOCH>",
        "<EPOCH>123.5</EPOCH>",
    );
    crate::common::assert_invalid_epoch(&Omm::from_xml(&numeric_mean_epoch).unwrap_err(), "123.5");
}
