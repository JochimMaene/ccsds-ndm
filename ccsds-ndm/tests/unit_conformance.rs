// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::Ndm;

#[test]
fn cdm_kvn_rejects_a_state_vector_with_metre_units() {
    let input = include_str!("../data/kvn/cdm_364.kvn").replacen(
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
        include_str!("../data/kvn/apm_g1.kvn").replacen("Q1 = 0.00005", "Q1 = 0.00005 [deg]", 1);
    Apm::from_kvn(&input).expect_err("quaternion components are dimensionless");
}

#[test]
fn xml_required_unit_attributes_are_not_inferred() {
    let input = include_str!("../data/xml/cdm_44.xml").replacen("<X units=\"km\">", "<X>", 1);
    Cdm::from_xml(&input).expect_err("CDM XML state-vector positions require units");
}
