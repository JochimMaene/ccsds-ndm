// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::Rdm;
use crate::error::Result;

pub(super) fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    const HEADER: &[&[u8]] = &[b"COMMENT", b"CREATION_DATE", b"ORIGINATOR", b"MESSAGE_ID"];
    const METADATA: &[&[u8]] = &[
        b"COMMENT",
        b"OBJECT_NAME",
        b"INTERNATIONAL_DESIGNATOR",
        b"CATALOG_NAME",
        b"OBJECT_DESIGNATOR",
        b"OBJECT_TYPE",
        b"OBJECT_OWNER",
        b"OBJECT_OPERATOR",
        b"CONTROLLED_REENTRY",
        b"CENTER_NAME",
        b"TIME_SYSTEM",
        b"EPOCH_TZERO",
        b"REF_FRAME",
        b"REF_FRAME_EPOCH",
        b"EPHEMERIS_NAME",
        b"GRAVITY_MODEL",
        b"ATMOSPHERIC_MODEL",
        b"SOLAR_FLUX_PREDICTION",
        b"N_BODY_PERTURBATIONS",
        b"SOLAR_RAD_PRESSURE",
        b"EARTH_TIDES",
        b"INTRACK_THRUST",
        b"DRAG_PARAMETERS_SOURCE",
        b"DRAG_PARAMETERS_ALTITUDE",
        b"REENTRY_UNCERTAINTY_METHOD",
        b"REENTRY_DISINTEGRATION",
        b"IMPACT_UNCERTAINTY_METHOD",
        b"PREVIOUS_MESSAGE_ID",
        b"PREVIOUS_MESSAGE_EPOCH",
        b"NEXT_MESSAGE_EPOCH",
    ];
    const DATA: &[&[u8]] = &[
        b"COMMENT",
        b"atmosphericReentryParameters",
        b"groundImpactParameters",
        b"stateVector",
        b"covarianceMatrix",
        b"spacecraftParameters",
        b"odParameters",
        b"userDefinedParameters",
    ];
    const ATMOSPHERIC: &[&[u8]] = &[
        b"COMMENT",
        b"ORBIT_LIFETIME",
        b"REENTRY_ALTITUDE",
        b"ORBIT_LIFETIME_WINDOW_START",
        b"ORBIT_LIFETIME_WINDOW_END",
        b"NOMINAL_REENTRY_EPOCH",
        b"REENTRY_WINDOW_START",
        b"REENTRY_WINDOW_END",
        b"ORBIT_LIFETIME_CONFIDENCE_LEVEL",
    ];
    const GROUND: &[&[u8]] = &[
        b"COMMENT",
        b"PROBABILITY_OF_IMPACT",
        b"PROBABILITY_OF_BURN_UP",
        b"PROBABILITY_OF_BREAK_UP",
        b"PROBABILITY_OF_LAND_IMPACT",
        b"PROBABILITY_OF_CASUALTY",
        b"NOMINAL_IMPACT_EPOCH",
        b"IMPACT_WINDOW_START",
        b"IMPACT_WINDOW_END",
        b"IMPACT_REF_FRAME",
        b"NOMINAL_IMPACT_LON",
        b"NOMINAL_IMPACT_LAT",
        b"NOMINAL_IMPACT_ALT",
        b"IMPACT_1_CONFIDENCE",
        b"IMPACT_1_START_LON",
        b"IMPACT_1_START_LAT",
        b"IMPACT_1_STOP_LON",
        b"IMPACT_1_STOP_LAT",
        b"IMPACT_1_CROSS_TRACK",
        b"IMPACT_2_CONFIDENCE",
        b"IMPACT_2_START_LON",
        b"IMPACT_2_START_LAT",
        b"IMPACT_2_STOP_LON",
        b"IMPACT_2_STOP_LAT",
        b"IMPACT_2_CROSS_TRACK",
        b"IMPACT_3_CONFIDENCE",
        b"IMPACT_3_START_LON",
        b"IMPACT_3_START_LAT",
        b"IMPACT_3_STOP_LON",
        b"IMPACT_3_STOP_LAT",
        b"IMPACT_3_CROSS_TRACK",
    ];
    const STATE: &[&[u8]] = &[
        b"COMMENT", b"EPOCH", b"X", b"Y", b"Z", b"X_DOT", b"Y_DOT", b"Z_DOT",
    ];
    const COVARIANCE: &[&[u8]] = &[
        b"COMMENT",
        b"COV_REF_FRAME",
        b"CX_X",
        b"CY_X",
        b"CY_Y",
        b"CZ_X",
        b"CZ_Y",
        b"CZ_Z",
        b"CX_DOT_X",
        b"CX_DOT_Y",
        b"CX_DOT_Z",
        b"CX_DOT_X_DOT",
        b"CY_DOT_X",
        b"CY_DOT_Y",
        b"CY_DOT_Z",
        b"CY_DOT_X_DOT",
        b"CY_DOT_Y_DOT",
        b"CZ_DOT_X",
        b"CZ_DOT_Y",
        b"CZ_DOT_Z",
        b"CZ_DOT_X_DOT",
        b"CZ_DOT_Y_DOT",
        b"CZ_DOT_Z_DOT",
    ];
    const SPACECRAFT: &[&[u8]] = &[
        b"COMMENT",
        b"WET_MASS",
        b"DRY_MASS",
        b"HAZARDOUS_SUBSTANCES",
        b"SOLAR_RAD_AREA",
        b"SOLAR_RAD_COEFF",
        b"DRAG_AREA",
        b"DRAG_COEFF",
        b"RCS",
        b"BALLISTIC_COEFF",
        b"THRUST_ACCELERATION",
    ];
    const OD: &[&[u8]] = &[
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
    ];
    const USER_DEFINED: &[&[u8]] = &[b"COMMENT", b"USER_DEFINED"];

    fn in_sequence(child: &[u8], sequence: &[&[u8]]) -> Option<XmlSequenceRule> {
        rank_of(child, sequence)
            .map(|rank| XmlSequenceRule::new(rank, matches!(child, b"COMMENT" | b"USER_DEFINED")))
    }

    /// `userDefinedType` wraps its children in a repeating sequence, so a COMMENT may open a new
    /// iteration after a USER_DEFINED.
    fn in_repeating_sequence(child: &[u8], sequence: &[&[u8]]) -> Option<XmlSequenceRule> {
        rank_of(child, sequence).map(|rank| {
            XmlSequenceRule::restarting(rank, matches!(child, b"COMMENT" | b"USER_DEFINED"))
        })
    }

    fn rank_of(child: &[u8], sequence: &[&[u8]]) -> Option<u16> {
        sequence
            .iter()
            .position(|candidate| *candidate == child)
            .map(|rank| rank as u16)
    }

    crate::xml::validate_element_sequences(
        xml,
        "RDM",
        |parent, child| match parent {
            b"rdm" => in_sequence(child, &[b"header", b"body"]),
            b"header" => in_sequence(child, HEADER),
            b"body" => in_sequence(child, &[b"segment"]),
            b"segment" => in_sequence(child, &[b"metadata", b"data"]),
            b"metadata" => in_sequence(child, METADATA),
            b"data" => in_sequence(child, DATA),
            b"atmosphericReentryParameters" => in_sequence(child, ATMOSPHERIC),
            b"groundImpactParameters" => in_sequence(child, GROUND),
            b"stateVector" => in_sequence(child, STATE),
            b"covarianceMatrix" => in_sequence(child, COVARIANCE),
            b"spacecraftParameters" => in_sequence(child, SPACECRAFT),
            b"odParameters" => in_sequence(child, OD),
            b"userDefinedParameters" => in_repeating_sequence(child, USER_DEFINED),
            _ => None,
        },
        |element, attribute| match attribute {
            b"parameter" => element == b"USER_DEFINED",
            b"units" => matches!(
                element,
                b"DRAG_PARAMETERS_ALTITUDE"
                    | b"ORBIT_LIFETIME"
                    | b"REENTRY_ALTITUDE"
                    | b"ORBIT_LIFETIME_WINDOW_START"
                    | b"ORBIT_LIFETIME_WINDOW_END"
                    | b"ORBIT_LIFETIME_CONFIDENCE_LEVEL"
                    | b"NOMINAL_IMPACT_LON"
                    | b"NOMINAL_IMPACT_LAT"
                    | b"NOMINAL_IMPACT_ALT"
                    | b"IMPACT_1_CONFIDENCE"
                    | b"IMPACT_1_START_LON"
                    | b"IMPACT_1_START_LAT"
                    | b"IMPACT_1_STOP_LON"
                    | b"IMPACT_1_STOP_LAT"
                    | b"IMPACT_1_CROSS_TRACK"
                    | b"IMPACT_2_CONFIDENCE"
                    | b"IMPACT_2_START_LON"
                    | b"IMPACT_2_START_LAT"
                    | b"IMPACT_2_STOP_LON"
                    | b"IMPACT_2_STOP_LAT"
                    | b"IMPACT_2_CROSS_TRACK"
                    | b"IMPACT_3_CONFIDENCE"
                    | b"IMPACT_3_START_LON"
                    | b"IMPACT_3_START_LAT"
                    | b"IMPACT_3_STOP_LON"
                    | b"IMPACT_3_STOP_LAT"
                    | b"IMPACT_3_CROSS_TRACK"
                    | b"X"
                    | b"Y"
                    | b"Z"
                    | b"X_DOT"
                    | b"Y_DOT"
                    | b"Z_DOT"
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
                    | b"WET_MASS"
                    | b"DRY_MASS"
                    | b"SOLAR_RAD_AREA"
                    | b"DRAG_AREA"
                    | b"RCS"
                    | b"BALLISTIC_COEFF"
                    | b"THRUST_ACCELERATION"
                    | b"RECOMMENDED_OD_SPAN"
                    | b"ACTUAL_OD_SPAN"
                    | b"RESIDUALS_ACCEPTED"
            ),
            _ => false,
        },
    )
}

impl Rdm {
    /// XML-only representability: `NOMINAL_IMPACT_ALT` is unbounded in RDM but the common 4.0
    /// XSD's `altRange` is Earth-derived, so a book-valid altitude for a non-Earth body cannot be
    /// expressed in this XML edition. The model keeps the value; conversion is refused rather
    /// than the value being altered to fit.
    pub(crate) fn validate_xml_representability(&self) -> Result<()> {
        let Some(parameters) = &self.body.segment.data.ground_impact_parameters else {
            return Ok(());
        };
        let Some(altitude) = &parameters.nominal_impact_alt else {
            return Ok(());
        };
        if altitude.is_xml_representable() {
            return Ok(());
        }
        Err(crate::error::ValidationError::OutOfRange {
            name: "NOMINAL_IMPACT_ALT".into(),
            value: altitude.value.to_string(),
            expected: "[-430.5, 8848] for the 4.0 XML edition".into(),
            line: None,
        }
        .at_path("body.segment.data.ground_impact_parameters.nominal_impact_alt")
        .into())
    }
}
