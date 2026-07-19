// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::traits::{FromKvnFloat, Ndm};
use ccsds_ndm::types::{PositionRequired, PositionUnits};

#[test]
fn required_kvn_units_accept_only_the_ccsds_unit() {
    let position = PositionRequired::from_kvn_float(42.0, Some("km"))
        .expect("CCSDS position units should be accepted");
    assert_eq!(position.units, PositionUnits::Km);

    let inferred = PositionRequired::from_kvn_float(42.0, None)
        .expect("KVN permits an omitted documented unit");
    assert_eq!(inferred.units, PositionUnits::Km);

    let error = PositionRequired::from_kvn_float(42.0, Some("m"))
        .expect_err("a non-CCSDS position unit must not be reinterpreted as kilometres");
    assert!(error.to_string().contains("expected one of: \"km\""));
}

#[test]
fn cdm_kvn_rejects_a_state_vector_with_metre_units() {
    let input = include_str!("../../data/kvn/cdm_364.kvn").replacen(
        "X = -41600.46272465 [km]",
        "X = -41600.46272465 [m]",
        1,
    );
    Cdm::from_kvn(&input)
        .expect_err("CDM state-vector positions are kilometres, not arbitrary length units");
}

#[test]
fn dimensionless_kvn_values_reject_spurious_units() {
    let input =
        include_str!("../../data/kvn/apm_g1.kvn").replacen("Q1 = 0.00005", "Q1 = 0.00005 [deg]", 1);
    Apm::from_kvn(&input).expect_err("quaternion components are dimensionless");
}

#[test]
fn xml_required_unit_attributes_are_not_inferred() {
    let input = include_str!("../../data/xml/cdm_44.xml").replacen("<X units=\"km\">", "<X>", 1);
    Cdm::from_xml(&input).expect_err("CDM XML state-vector positions require units");
}
