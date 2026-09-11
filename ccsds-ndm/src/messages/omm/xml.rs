// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::Result;

pub(super) fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    let rule = |rank, repeatable| XmlSequenceRule::new(rank, repeatable);
    // `userDefinedType` wraps its children in a repeating sequence, so a COMMENT may open a new
    // iteration after a USER_DEFINED.
    let repeating = |rank, repeatable| XmlSequenceRule::restarting(rank, repeatable);
    crate::xml::validate_element_sequences(
        xml,
        "OMM",
        |parent, child| {
            Some(match (parent, child) {
                (b"omm", b"header") => rule(0, false),
                (b"omm", b"body") => rule(1, false),
                (b"header", b"COMMENT") => rule(0, true),
                (b"header", b"CLASSIFICATION") => rule(1, false),
                (b"header", b"CREATION_DATE") => rule(2, false),
                (b"header", b"ORIGINATOR") => rule(3, false),
                (b"header", b"MESSAGE_ID") => rule(4, false),
                (b"body", b"segment") => rule(0, false),
                (b"segment", b"metadata") => rule(0, false),
                (b"segment", b"data") => rule(1, false),
                (b"metadata", b"COMMENT") => rule(0, true),
                (b"metadata", b"OBJECT_NAME") => rule(1, false),
                (b"metadata", b"OBJECT_ID") => rule(2, false),
                (b"metadata", b"CENTER_NAME") => rule(3, false),
                (b"metadata", b"REF_FRAME") => rule(4, false),
                (b"metadata", b"REF_FRAME_EPOCH") => rule(5, false),
                (b"metadata", b"TIME_SYSTEM") => rule(6, false),
                (b"metadata", b"MEAN_ELEMENT_THEORY") => rule(7, false),
                (b"data", b"COMMENT") => rule(0, true),
                (b"data", b"meanElements") => rule(1, false),
                (b"data", b"spacecraftParameters") => rule(2, false),
                (b"data", b"tleParameters") => rule(3, false),
                (b"data", b"covarianceMatrix") => rule(4, false),
                (b"data", b"userDefinedParameters") => rule(5, false),
                (b"meanElements", b"COMMENT") => rule(0, true),
                (b"meanElements", b"EPOCH") => rule(1, false),
                (b"meanElements", b"SEMI_MAJOR_AXIS" | b"MEAN_MOTION") => rule(2, false),
                (b"meanElements", b"ECCENTRICITY") => rule(3, false),
                (b"meanElements", b"INCLINATION") => rule(4, false),
                (b"meanElements", b"RA_OF_ASC_NODE") => rule(5, false),
                (b"meanElements", b"ARG_OF_PERICENTER") => rule(6, false),
                (b"meanElements", b"MEAN_ANOMALY") => rule(7, false),
                (b"meanElements", b"GM") => rule(8, false),
                (b"spacecraftParameters", b"COMMENT") => rule(0, true),
                (b"spacecraftParameters", b"MASS") => rule(1, false),
                (b"spacecraftParameters", b"SOLAR_RAD_AREA") => rule(2, false),
                (b"spacecraftParameters", b"SOLAR_RAD_COEFF") => rule(3, false),
                (b"spacecraftParameters", b"DRAG_AREA") => rule(4, false),
                (b"spacecraftParameters", b"DRAG_COEFF") => rule(5, false),
                (b"tleParameters", b"COMMENT") => rule(0, true),
                (b"tleParameters", b"EPHEMERIS_TYPE") => rule(1, false),
                (b"tleParameters", b"CLASSIFICATION_TYPE") => rule(2, false),
                (b"tleParameters", b"NORAD_CAT_ID") => rule(3, false),
                (b"tleParameters", b"ELEMENT_SET_NO") => rule(4, false),
                (b"tleParameters", b"REV_AT_EPOCH") => rule(5, false),
                (b"tleParameters", b"BSTAR" | b"BTERM") => rule(6, false),
                (b"tleParameters", b"MEAN_MOTION_DOT") => rule(7, false),
                (b"tleParameters", b"MEAN_MOTION_DDOT" | b"AGOM") => rule(8, false),
                (b"covarianceMatrix", b"COMMENT") => rule(0, true),
                (b"covarianceMatrix", b"COV_REF_FRAME") => rule(1, false),
                (b"covarianceMatrix", b"CX_X") => rule(2, false),
                (b"covarianceMatrix", b"CY_X") => rule(3, false),
                (b"covarianceMatrix", b"CY_Y") => rule(4, false),
                (b"covarianceMatrix", b"CZ_X") => rule(5, false),
                (b"covarianceMatrix", b"CZ_Y") => rule(6, false),
                (b"covarianceMatrix", b"CZ_Z") => rule(7, false),
                (b"covarianceMatrix", b"CX_DOT_X") => rule(8, false),
                (b"covarianceMatrix", b"CX_DOT_Y") => rule(9, false),
                (b"covarianceMatrix", b"CX_DOT_Z") => rule(10, false),
                (b"covarianceMatrix", b"CX_DOT_X_DOT") => rule(11, false),
                (b"covarianceMatrix", b"CY_DOT_X") => rule(12, false),
                (b"covarianceMatrix", b"CY_DOT_Y") => rule(13, false),
                (b"covarianceMatrix", b"CY_DOT_Z") => rule(14, false),
                (b"covarianceMatrix", b"CY_DOT_X_DOT") => rule(15, false),
                (b"covarianceMatrix", b"CY_DOT_Y_DOT") => rule(16, false),
                (b"covarianceMatrix", b"CZ_DOT_X") => rule(17, false),
                (b"covarianceMatrix", b"CZ_DOT_Y") => rule(18, false),
                (b"covarianceMatrix", b"CZ_DOT_Z") => rule(19, false),
                (b"covarianceMatrix", b"CZ_DOT_X_DOT") => rule(20, false),
                (b"covarianceMatrix", b"CZ_DOT_Y_DOT") => rule(21, false),
                (b"covarianceMatrix", b"CZ_DOT_Z_DOT") => rule(22, false),
                (b"userDefinedParameters", b"COMMENT") => repeating(0, true),
                (b"userDefinedParameters", b"USER_DEFINED") => repeating(1, true),
                _ => return None,
            })
        },
        |element, attribute| match attribute {
            b"units" => matches!(
                element,
                b"SEMI_MAJOR_AXIS"
                    | b"MEAN_MOTION"
                    | b"INCLINATION"
                    | b"RA_OF_ASC_NODE"
                    | b"ARG_OF_PERICENTER"
                    | b"MEAN_ANOMALY"
                    | b"GM"
                    | b"MASS"
                    | b"SOLAR_RAD_AREA"
                    | b"DRAG_AREA"
                    | b"BSTAR"
                    | b"BTERM"
                    | b"MEAN_MOTION_DOT"
                    | b"MEAN_MOTION_DDOT"
                    | b"AGOM"
                    | b"CX_X"
                    | b"CY_X"
                    | b"CY_Y"
                    | b"CZ_X"
                    | b"CZ_Y"
                    | b"CZ_Z"
                    | b"CX_DOT_X"
                    | b"CX_DOT_Y"
                    | b"CX_DOT_Z"
                    | b"CX_DOT_X_DOT"
                    | b"CY_DOT_X"
                    | b"CY_DOT_Y"
                    | b"CY_DOT_Z"
                    | b"CY_DOT_X_DOT"
                    | b"CY_DOT_Y_DOT"
                    | b"CZ_DOT_X"
                    | b"CZ_DOT_Y"
                    | b"CZ_DOT_Z"
                    | b"CZ_DOT_X_DOT"
                    | b"CZ_DOT_Y_DOT"
                    | b"CZ_DOT_Z_DOT"
            ),
            b"parameter" => element == b"USER_DEFINED",
            _ => false,
        },
    )
}
