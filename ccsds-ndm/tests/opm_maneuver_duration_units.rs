// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::TimeUnits;

const OPM_3_WITH_MANEUVER: &str = include_str!("../../data/kvn/opm_g2.kvn");

#[test]
fn opm_xml_generation_rejects_day_as_maneuver_duration_unit() {
    let mut opm = Opm::from_kvn(OPM_3_WITH_MANEUVER).expect("failed to parse OPM 3.0 fixture");
    opm.body.segment.data.maneuver_parameters[0]
        .man_duration
        .units = Some(TimeUnits::Day);

    let error = opm
        .to_xml()
        .expect_err("day is not an XSD-valid duration unit");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.maneuver_parameters.man_duration.units")
    );
    assert!(error.as_validation_error().is_some());
}

#[test]
fn opm_xml_generation_accepts_seconds_as_maneuver_duration_unit() {
    let mut opm = Opm::from_kvn(OPM_3_WITH_MANEUVER).expect("failed to parse OPM 3.0 fixture");
    opm.body.segment.data.maneuver_parameters[0]
        .man_duration
        .units = Some(TimeUnits::Seconds);

    let xml = opm.to_xml().expect("seconds is the XSD duration unit");
    assert!(xml.contains("<MAN_DURATION units=\"s\">"));
    assert!(!xml.contains("<MAN_DURATION units=\"d\">"));
}
