// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::Result;

pub(super) fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    let rule = |rank, repeatable| XmlSequenceRule::new(rank, repeatable);
    crate::xml::validate_element_sequences(
        xml,
        "APM",
        |parent, child| {
            Some(match (parent, child) {
                (b"apm", b"header") => rule(0, false),
                (b"apm", b"body") => rule(1, false),
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
                (b"metadata", b"TIME_SYSTEM") => rule(4, false),
                (b"data", b"COMMENT") => rule(0, true),
                (b"data", b"EPOCH") => rule(1, false),
                (b"data", b"quaternionState") => rule(2, true),
                (b"data", b"eulerAngleState") => rule(3, true),
                (b"data", b"angularVelocity") => rule(4, true),
                (b"data", b"spin") => rule(5, true),
                (b"data", b"inertia") => rule(6, true),
                (b"data", b"maneuverParameters") => rule(7, true),
                (b"quaternionState", b"COMMENT") => rule(0, true),
                (b"quaternionState", b"REF_FRAME_A") => rule(1, false),
                (b"quaternionState", b"REF_FRAME_B") => rule(2, false),
                (b"quaternionState", b"quaternion") => rule(3, false),
                (b"quaternionState", b"quaternionDot") => rule(4, false),
                (b"quaternion", b"Q1") => rule(0, false),
                (b"quaternion", b"Q2") => rule(1, false),
                (b"quaternion", b"Q3") => rule(2, false),
                (b"quaternion", b"QC") => rule(3, false),
                (b"quaternionDot", b"Q1_DOT") => rule(0, false),
                (b"quaternionDot", b"Q2_DOT") => rule(1, false),
                (b"quaternionDot", b"Q3_DOT") => rule(2, false),
                (b"quaternionDot", b"QC_DOT") => rule(3, false),
                (b"eulerAngleState", b"COMMENT") => rule(0, true),
                (b"eulerAngleState", b"REF_FRAME_A") => rule(1, false),
                (b"eulerAngleState", b"REF_FRAME_B") => rule(2, false),
                (b"eulerAngleState", b"EULER_ROT_SEQ") => rule(3, false),
                (b"eulerAngleState", b"ANGLE_1") => rule(4, false),
                (b"eulerAngleState", b"ANGLE_2") => rule(5, false),
                (b"eulerAngleState", b"ANGLE_3") => rule(6, false),
                (b"eulerAngleState", b"ANGLE_1_DOT") => rule(7, false),
                (b"eulerAngleState", b"ANGLE_2_DOT") => rule(8, false),
                (b"eulerAngleState", b"ANGLE_3_DOT") => rule(9, false),
                (b"angularVelocity", b"COMMENT") => rule(0, true),
                (b"angularVelocity", b"REF_FRAME_A") => rule(1, false),
                (b"angularVelocity", b"REF_FRAME_B") => rule(2, false),
                (b"angularVelocity", b"ANGVEL_FRAME") => rule(3, false),
                (b"angularVelocity", b"ANGVEL_X") => rule(4, false),
                (b"angularVelocity", b"ANGVEL_Y") => rule(5, false),
                (b"angularVelocity", b"ANGVEL_Z") => rule(6, false),
                (b"spin", b"COMMENT") => rule(0, true),
                (b"spin", b"REF_FRAME_A") => rule(1, false),
                (b"spin", b"REF_FRAME_B") => rule(2, false),
                (b"spin", b"SPIN_ALPHA") => rule(3, false),
                (b"spin", b"SPIN_DELTA") => rule(4, false),
                (b"spin", b"SPIN_ANGLE") => rule(5, false),
                (b"spin", b"SPIN_ANGLE_VEL") => rule(6, false),
                (b"spin", b"NUTATION") => rule(7, false),
                (b"spin", b"NUTATION_PER") => rule(8, false),
                (b"spin", b"NUTATION_PHASE") => rule(9, false),
                (b"spin", b"MOMENTUM_ALPHA") => rule(10, false),
                (b"spin", b"MOMENTUM_DELTA") => rule(11, false),
                (b"spin", b"NUTATION_VEL") => rule(12, false),
                (b"inertia", b"COMMENT") => rule(0, true),
                (b"inertia", b"INERTIA_REF_FRAME") => rule(1, false),
                (b"inertia", b"IXX") => rule(2, false),
                (b"inertia", b"IYY") => rule(3, false),
                (b"inertia", b"IZZ") => rule(4, false),
                (b"inertia", b"IXY") => rule(5, false),
                (b"inertia", b"IXZ") => rule(6, false),
                (b"inertia", b"IYZ") => rule(7, false),
                (b"maneuverParameters", b"COMMENT") => rule(0, true),
                (b"maneuverParameters", b"MAN_EPOCH_START") => rule(1, false),
                (b"maneuverParameters", b"MAN_DURATION") => rule(2, false),
                (b"maneuverParameters", b"MAN_REF_FRAME") => rule(3, false),
                (b"maneuverParameters", b"MAN_TOR_X") => rule(4, false),
                (b"maneuverParameters", b"MAN_TOR_Y") => rule(5, false),
                (b"maneuverParameters", b"MAN_TOR_Z") => rule(6, false),
                (b"maneuverParameters", b"MAN_DELTA_MASS") => rule(7, false),
                _ => return None,
            })
        },
        |element, attribute| {
            attribute == b"units"
                && matches!(
                    element,
                    b"Q1_DOT"
                        | b"Q2_DOT"
                        | b"Q3_DOT"
                        | b"QC_DOT"
                        | b"ANGLE_1"
                        | b"ANGLE_2"
                        | b"ANGLE_3"
                        | b"ANGLE_1_DOT"
                        | b"ANGLE_2_DOT"
                        | b"ANGLE_3_DOT"
                        | b"ANGVEL_X"
                        | b"ANGVEL_Y"
                        | b"ANGVEL_Z"
                        | b"SPIN_ALPHA"
                        | b"SPIN_DELTA"
                        | b"SPIN_ANGLE"
                        | b"SPIN_ANGLE_VEL"
                        | b"NUTATION"
                        | b"NUTATION_PER"
                        | b"NUTATION_PHASE"
                        | b"MOMENTUM_ALPHA"
                        | b"MOMENTUM_DELTA"
                        | b"NUTATION_VEL"
                        | b"IXX"
                        | b"IYY"
                        | b"IZZ"
                        | b"IXY"
                        | b"IXZ"
                        | b"IYZ"
                        | b"MAN_DURATION"
                        | b"MAN_TOR_X"
                        | b"MAN_TOR_Y"
                        | b"MAN_TOR_Z"
                        | b"MAN_DELTA_MASS"
                )
        },
    )
}
