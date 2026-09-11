// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::{FieldPath, Opm, OpmBody};
use crate::common::OdmHeader;
use crate::error::{Result, ValidationError};
use serde::Serialize;

impl Serialize for Opm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename = "opm")]
        struct XmlOpm<'a> {
            #[serde(rename = "@xmlns:xsi")]
            xmlns_xsi: &'static str,
            #[serde(rename = "@id")]
            id: &'a Option<String>,
            #[serde(rename = "@version")]
            version: &'a str,
            header: &'a OdmHeader,
            body: &'a OpmBody,
        }

        XmlOpm {
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance",
            id: &self.id,
            version: &self.version,
            header: &self.header,
            body: &self.body,
        }
        .serialize(serializer)
    }
}

fn validate_xml_envelope(xml: &str, source_edition: &mut Option<String>) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    const OPM: &[&[u8]] = &[b"header", b"body"];
    const HEADER: &[&[u8]] = &[
        b"COMMENT",
        b"CLASSIFICATION",
        b"CREATION_DATE",
        b"ORIGINATOR",
        b"MESSAGE_ID",
    ];
    const BODY: &[&[u8]] = &[b"segment"];
    const SEGMENT: &[&[u8]] = &[b"metadata", b"data"];
    const METADATA: &[&[u8]] = &[
        b"COMMENT",
        b"OBJECT_NAME",
        b"OBJECT_ID",
        b"CENTER_NAME",
        b"REF_FRAME",
        b"REF_FRAME_EPOCH",
        b"TIME_SYSTEM",
    ];
    const DATA: &[&[u8]] = &[
        b"COMMENT",
        b"stateVector",
        b"keplerianElements",
        b"spacecraftParameters",
        b"covarianceMatrix",
        b"maneuverParameters",
        b"userDefinedParameters",
    ];
    const STATE_VECTOR: &[&[u8]] = &[
        b"COMMENT", b"EPOCH", b"X", b"Y", b"Z", b"X_DOT", b"Y_DOT", b"Z_DOT",
    ];
    const KEPLERIAN: &[&[u8]] = &[
        b"COMMENT",
        b"SEMI_MAJOR_AXIS",
        b"ECCENTRICITY",
        b"INCLINATION",
        b"RA_OF_ASC_NODE",
        b"ARG_OF_PERICENTER",
        b"TRUE_ANOMALY",
        b"MEAN_ANOMALY",
        b"GM",
    ];
    const SPACECRAFT: &[&[u8]] = &[
        b"COMMENT",
        b"MASS",
        b"SOLAR_RAD_AREA",
        b"SOLAR_RAD_COEFF",
        b"DRAG_AREA",
        b"DRAG_COEFF",
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
    const MANEUVER: &[&[u8]] = &[
        b"COMMENT",
        b"MAN_EPOCH_IGNITION",
        b"MAN_DURATION",
        b"MAN_DELTA_MASS",
        b"MAN_REF_FRAME",
        b"MAN_DV_1",
        b"MAN_DV_2",
        b"MAN_DV_3",
    ];
    const USER_DEFINED: &[&[u8]] = &[b"COMMENT", b"USER_DEFINED"];

    fn rule(parent: &[u8], child: &[u8]) -> Option<XmlSequenceRule> {
        let sequence = match parent {
            b"opm" => OPM,
            b"header" => HEADER,
            b"body" => BODY,
            b"segment" => SEGMENT,
            b"metadata" => METADATA,
            b"data" => DATA,
            b"stateVector" => STATE_VECTOR,
            b"keplerianElements" => KEPLERIAN,
            b"spacecraftParameters" => SPACECRAFT,
            b"covarianceMatrix" => COVARIANCE,
            b"maneuverParameters" => MANEUVER,
            b"userDefinedParameters" => USER_DEFINED,
            _ => return None,
        };
        sequence
            .iter()
            .position(|candidate| *candidate == child)
            .map(|rank| {
                let repeatable = child == b"COMMENT"
                    || child == b"maneuverParameters"
                    || child == b"USER_DEFINED";
                // `userDefinedType` wraps its children in a repeating sequence, so a COMMENT
                // may open a new iteration after a USER_DEFINED.
                if parent == b"userDefinedParameters" {
                    XmlSequenceRule::restarting(rank as u16, repeatable)
                } else {
                    XmlSequenceRule::new(rank as u16, repeatable)
                }
            })
    }

    crate::xml::validate_standalone_document(
        xml,
        b"opm",
        "OPM",
        source_edition,
        crate::xml::MessageSchema {
            child_rule: rule,
            attribute_allowed: |element: &[u8], attribute: &[u8]| match attribute {
                b"units" => matches!(
                    element,
                    b"X" | b"Y"
                        | b"Z"
                        | b"X_DOT"
                        | b"Y_DOT"
                        | b"Z_DOT"
                        | b"SEMI_MAJOR_AXIS"
                        | b"INCLINATION"
                        | b"RA_OF_ASC_NODE"
                        | b"ARG_OF_PERICENTER"
                        | b"TRUE_ANOMALY"
                        | b"MEAN_ANOMALY"
                        | b"GM"
                        | b"MASS"
                        | b"SOLAR_RAD_AREA"
                        | b"DRAG_AREA"
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
                        | b"MAN_DURATION"
                        | b"MAN_DELTA_MASS"
                        | b"MAN_DV_1"
                        | b"MAN_DV_2"
                        | b"MAN_DV_3"
                ),
                b"nil" | b"xsi:nil" => matches!(
                    element,
                    b"REF_FRAME_EPOCH"
                        | b"TRUE_ANOMALY"
                        | b"MEAN_ANOMALY"
                        | b"MASS"
                        | b"SOLAR_RAD_AREA"
                        | b"SOLAR_RAD_COEFF"
                        | b"DRAG_AREA"
                        | b"DRAG_COEFF"
                        | b"COV_REF_FRAME"
                ),
                b"parameter" => element == b"USER_DEFINED",
                _ => false,
            },
        },
    )
}

impl Opm {
    pub(crate) fn from_xml_strict(xml: &str) -> Result<Self> {
        let mut source_edition = None;
        (|| {
            validate_xml_envelope(xml, &mut source_edition)?;
            let opm: Self = crate::xml::from_str_with_context(xml, "OPM")?;
            crate::traits::Validate::validate(&opm)?;
            Ok(opm)
        })()
        .map_err(|error: crate::error::CcsdsNdmError| {
            error.with_parse_context(
                crate::validation::MessageKind::Opm,
                crate::error::DiagnosticNotation::Xml,
                xml,
                source_edition.as_deref(),
            )
        })
    }

    pub(crate) fn validate_xml_text(&self) -> Result<()> {
        match self.xml_text_errors().into_iter().next() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }

    fn xml_text_errors(&self) -> Vec<ValidationError> {
        fn check(
            errors: &mut Vec<ValidationError>,
            field: &'static str,
            value: &str,
            path: impl Into<FieldPath>,
        ) {
            if let Some(error) = crate::validation::xml_text_error(field, value) {
                errors.push(error.at_path(path.into().resolve()));
            }
        }
        fn check_comments(
            errors: &mut Vec<ValidationError>,
            comments: &[String],
            path: impl Into<FieldPath>,
        ) {
            let path = path.into();
            for comment in comments {
                check(errors, "COMMENT", comment, path);
            }
        }

        let mut errors = Vec::new();
        check_comments(&mut errors, &self.header.comment, "header.comment");
        if let Some(value) = &self.header.classification {
            check(
                &mut errors,
                "CLASSIFICATION",
                value,
                "header.classification",
            );
        }
        check(
            &mut errors,
            "ORIGINATOR",
            &self.header.originator,
            "header.originator",
        );
        if let Some(value) = &self.header.message_id {
            check(&mut errors, "MESSAGE_ID", value, "header.message_id");
        }

        let segment = &self.body.segment;
        check_comments(
            &mut errors,
            &segment.metadata.comment,
            "body.segment.metadata.comment",
        );
        check(
            &mut errors,
            "OBJECT_NAME",
            &segment.metadata.object_name,
            "body.segment.metadata.object_name",
        );
        check(
            &mut errors,
            "OBJECT_ID",
            &segment.metadata.object_id,
            "body.segment.metadata.object_id",
        );
        check(
            &mut errors,
            "CENTER_NAME",
            &segment.metadata.center_name,
            "body.segment.metadata.center_name",
        );
        check(
            &mut errors,
            "REF_FRAME",
            &segment.metadata.ref_frame,
            "body.segment.metadata.ref_frame",
        );
        check(
            &mut errors,
            "TIME_SYSTEM",
            &segment.metadata.time_system,
            "body.segment.metadata.time_system",
        );

        let data = &segment.data;
        check_comments(&mut errors, &data.comment, "body.segment.data.comment");
        check_comments(
            &mut errors,
            &data.state_vector.comment,
            "body.segment.data.state_vector.comment",
        );
        if let Some(elements) = &data.keplerian_elements {
            check_comments(
                &mut errors,
                &elements.comment,
                "body.segment.data.keplerian_elements.comment",
            );
        }
        if let Some(parameters) = &data.spacecraft_parameters {
            check_comments(
                &mut errors,
                &parameters.comment,
                "body.segment.data.spacecraft_parameters.comment",
            );
        }
        if let Some(covariance) = &data.covariance_matrix {
            check_comments(
                &mut errors,
                &covariance.comment,
                "body.segment.data.covariance_matrix.comment",
            );
            if let Some(value) = &covariance.cov_ref_frame {
                check(
                    &mut errors,
                    "COV_REF_FRAME",
                    value,
                    "body.segment.data.covariance_matrix.cov_ref_frame",
                );
            }
        }
        for (index, maneuver) in data.maneuver_parameters.iter().enumerate() {
            check_comments(
                &mut errors,
                &maneuver.comment,
                FieldPath::Maneuver(index, "comment"),
            );
            check(
                &mut errors,
                "MAN_REF_FRAME",
                &maneuver.man_ref_frame,
                FieldPath::Maneuver(index, "man_ref_frame"),
            );
        }
        if let Some(user_defined) = &data.user_defined_parameters {
            check_comments(
                &mut errors,
                &user_defined.comment,
                "body.segment.data.user_defined_parameters.comment",
            );
            for parameter in &user_defined.user_defined {
                check(
                    &mut errors,
                    "USER_DEFINED parameter",
                    &parameter.parameter,
                    "body.segment.data.user_defined_parameters.user_defined.parameter",
                );
                check(
                    &mut errors,
                    "USER_DEFINED",
                    &parameter.value,
                    "body.segment.data.user_defined_parameters.user_defined.value",
                );
            }
        }
        errors
    }
}
