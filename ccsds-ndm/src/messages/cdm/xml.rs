// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::Result;

pub(super) fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    crate::xml::validate_element_sequences(
        xml,
        "CDM",
        |parent, child| {
            let children = cdm_xml_children(parent)?;
            let rank = children.iter().position(|candidate| *candidate == child)? as u16;
            let repeatable = child == b"COMMENT" || (parent == b"body" && child == b"segment");
            Some(XmlSequenceRule::new(rank, repeatable))
        },
        |element, attribute| {
            attribute == b"units"
                && matches!(
                    element,
                    b"MISS_DISTANCE"
                        | b"RELATIVE_SPEED"
                        | b"RELATIVE_POSITION_R"
                        | b"RELATIVE_POSITION_T"
                        | b"RELATIVE_POSITION_N"
                        | b"RELATIVE_VELOCITY_R"
                        | b"RELATIVE_VELOCITY_T"
                        | b"RELATIVE_VELOCITY_N"
                        | b"SCREEN_VOLUME_X"
                        | b"SCREEN_VOLUME_Y"
                        | b"SCREEN_VOLUME_Z"
                        | b"COLLISION_PROBABILITY"
                        | b"RECOMMENDED_OD_SPAN"
                        | b"ACTUAL_OD_SPAN"
                        | b"RESIDUALS_ACCEPTED"
                        | b"WEIGHTED_RMS"
                        | b"AREA_PC"
                        | b"AREA_DRG"
                        | b"AREA_SRP"
                        | b"MASS"
                        | b"CD_AREA_OVER_MASS"
                        | b"CR_AREA_OVER_MASS"
                        | b"THRUST_ACCELERATION"
                        | b"SEDR"
                        | b"X"
                        | b"Y"
                        | b"Z"
                        | b"X_DOT"
                        | b"Y_DOT"
                        | b"Z_DOT"
                        | b"CR_R"
                        | b"CT_R"
                        | b"CT_T"
                        | b"CN_R"
                        | b"CN_T"
                        | b"CN_N"
                        | b"CRDOT_R"
                        | b"CRDOT_T"
                        | b"CRDOT_N"
                        | b"CRDOT_RDOT"
                        | b"CTDOT_R"
                        | b"CTDOT_T"
                        | b"CTDOT_N"
                        | b"CTDOT_RDOT"
                        | b"CTDOT_TDOT"
                        | b"CNDOT_R"
                        | b"CNDOT_T"
                        | b"CNDOT_N"
                        | b"CNDOT_RDOT"
                        | b"CNDOT_TDOT"
                        | b"CNDOT_NDOT"
                        | b"CDRG_R"
                        | b"CDRG_T"
                        | b"CDRG_N"
                        | b"CDRG_RDOT"
                        | b"CDRG_TDOT"
                        | b"CDRG_NDOT"
                        | b"CDRG_DRG"
                        | b"CSRP_R"
                        | b"CSRP_T"
                        | b"CSRP_N"
                        | b"CSRP_RDOT"
                        | b"CSRP_TDOT"
                        | b"CSRP_NDOT"
                        | b"CSRP_DRG"
                        | b"CSRP_SRP"
                        | b"CTHR_R"
                        | b"CTHR_T"
                        | b"CTHR_N"
                        | b"CTHR_RDOT"
                        | b"CTHR_TDOT"
                        | b"CTHR_NDOT"
                        | b"CTHR_DRG"
                        | b"CTHR_SRP"
                        | b"CTHR_THR"
                )
        },
    )
}

fn cdm_xml_children(parent: &[u8]) -> Option<&'static [&'static [u8]]> {
    Some(match parent {
        b"cdm" => &[b"header", b"body"],
        b"header" => &[
            b"COMMENT",
            b"CREATION_DATE",
            b"ORIGINATOR",
            b"MESSAGE_FOR",
            b"MESSAGE_ID",
        ],
        b"body" => &[b"relativeMetadataData", b"segment"],
        b"relativeMetadataData" => &[
            b"COMMENT",
            b"TCA",
            b"MISS_DISTANCE",
            b"RELATIVE_SPEED",
            b"relativeStateVector",
            b"START_SCREEN_PERIOD",
            b"STOP_SCREEN_PERIOD",
            b"SCREEN_VOLUME_FRAME",
            b"SCREEN_VOLUME_SHAPE",
            b"SCREEN_VOLUME_X",
            b"SCREEN_VOLUME_Y",
            b"SCREEN_VOLUME_Z",
            b"SCREEN_ENTRY_TIME",
            b"SCREEN_EXIT_TIME",
            b"COLLISION_PROBABILITY",
            b"COLLISION_PROBABILITY_METHOD",
        ],
        b"relativeStateVector" => &[
            b"RELATIVE_POSITION_R",
            b"RELATIVE_POSITION_T",
            b"RELATIVE_POSITION_N",
            b"RELATIVE_VELOCITY_R",
            b"RELATIVE_VELOCITY_T",
            b"RELATIVE_VELOCITY_N",
        ],
        b"segment" => &[b"metadata", b"data"],
        b"metadata" => &[
            b"COMMENT",
            b"OBJECT",
            b"OBJECT_DESIGNATOR",
            b"CATALOG_NAME",
            b"OBJECT_NAME",
            b"INTERNATIONAL_DESIGNATOR",
            b"OBJECT_TYPE",
            b"OPERATOR_CONTACT_POSITION",
            b"OPERATOR_ORGANIZATION",
            b"OPERATOR_PHONE",
            b"OPERATOR_EMAIL",
            b"EPHEMERIS_NAME",
            b"COVARIANCE_METHOD",
            b"MANEUVERABLE",
            b"ORBIT_CENTER",
            b"REF_FRAME",
            b"GRAVITY_MODEL",
            b"ATMOSPHERIC_MODEL",
            b"N_BODY_PERTURBATIONS",
            b"SOLAR_RAD_PRESSURE",
            b"EARTH_TIDES",
            b"INTRACK_THRUST",
        ],
        b"data" => &[
            b"COMMENT",
            b"odParameters",
            b"additionalParameters",
            b"stateVector",
            b"covarianceMatrix",
        ],
        b"odParameters" => &[
            b"COMMENT",
            b"TIME_LASTOB_START",
            b"TIME_LASTOB_END",
            b"RECOMMENDED_OD_SPAN",
            b"ACTUAL_OD_SPAN",
            b"OBS_AVAILABLE",
            b"OBS_USED",
            b"TRACKS_AVAILABLE",
            b"TRACKS_USED",
            b"RESIDUALS_ACCEPTED",
            b"WEIGHTED_RMS",
        ],
        b"additionalParameters" => &[
            b"COMMENT",
            b"AREA_PC",
            b"AREA_DRG",
            b"AREA_SRP",
            b"MASS",
            b"CD_AREA_OVER_MASS",
            b"CR_AREA_OVER_MASS",
            b"THRUST_ACCELERATION",
            b"SEDR",
        ],
        b"stateVector" => &[b"COMMENT", b"X", b"Y", b"Z", b"X_DOT", b"Y_DOT", b"Z_DOT"],
        b"covarianceMatrix" => &[
            b"COMMENT",
            b"CR_R",
            b"CT_R",
            b"CT_T",
            b"CN_R",
            b"CN_T",
            b"CN_N",
            b"CRDOT_R",
            b"CRDOT_T",
            b"CRDOT_N",
            b"CRDOT_RDOT",
            b"CTDOT_R",
            b"CTDOT_T",
            b"CTDOT_N",
            b"CTDOT_RDOT",
            b"CTDOT_TDOT",
            b"CNDOT_R",
            b"CNDOT_T",
            b"CNDOT_N",
            b"CNDOT_RDOT",
            b"CNDOT_TDOT",
            b"CNDOT_NDOT",
            b"CDRG_R",
            b"CDRG_T",
            b"CDRG_N",
            b"CDRG_RDOT",
            b"CDRG_TDOT",
            b"CDRG_NDOT",
            b"CDRG_DRG",
            b"CSRP_R",
            b"CSRP_T",
            b"CSRP_N",
            b"CSRP_RDOT",
            b"CSRP_TDOT",
            b"CSRP_NDOT",
            b"CSRP_DRG",
            b"CSRP_SRP",
            b"CTHR_R",
            b"CTHR_T",
            b"CTHR_N",
            b"CTHR_RDOT",
            b"CTHR_TDOT",
            b"CTHR_NDOT",
            b"CTHR_DRG",
            b"CTHR_SRP",
            b"CTHR_THR",
        ],
        _ => return None,
    })
}
