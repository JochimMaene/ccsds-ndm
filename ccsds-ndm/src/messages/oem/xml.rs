// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::Oem;
use crate::error::Result;
use crate::types::{
    AccUnits, PositionCovarianceUnits, PositionUnits, PositionVelocityCovarianceUnits,
    VelocityCovarianceUnits, VelocityUnits,
};

pub(super) fn validate_envelope(xml: &str, source_edition: &mut Option<String>) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    const OEM: &[&[u8]] = &[b"header", b"body"];
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
        b"START_TIME",
        b"USEABLE_START_TIME",
        b"USEABLE_STOP_TIME",
        b"STOP_TIME",
        b"INTERPOLATION",
        b"INTERPOLATION_DEGREE",
    ];
    const DATA: &[&[u8]] = &[b"COMMENT", b"stateVector", b"covarianceMatrix"];
    const STATE_VECTOR: &[&[u8]] = &[
        b"EPOCH", b"X", b"Y", b"Z", b"X_DOT", b"Y_DOT", b"Z_DOT", b"X_DDOT", b"Y_DDOT", b"Z_DDOT",
    ];
    const COVARIANCE: &[&[u8]] = &[
        b"COMMENT",
        b"EPOCH",
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

    fn rule(parent: &[u8], child: &[u8]) -> Option<XmlSequenceRule> {
        let sequence = match parent {
            b"oem" => OEM,
            b"header" => HEADER,
            b"body" => BODY,
            b"segment" => SEGMENT,
            b"metadata" => METADATA,
            b"data" => DATA,
            b"stateVector" => STATE_VECTOR,
            b"covarianceMatrix" => COVARIANCE,
            _ => return None,
        };
        sequence
            .iter()
            .position(|candidate| *candidate == child)
            .map(|rank| {
                XmlSequenceRule::new(
                    rank as u16,
                    matches!(
                        child,
                        b"COMMENT" | b"segment" | b"stateVector" | b"covarianceMatrix"
                    ),
                )
            })
    }

    crate::xml::validate_standalone_document(
        xml,
        b"oem",
        "OEM",
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
                        | b"X_DDOT"
                        | b"Y_DDOT"
                        | b"Z_DDOT"
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
                _ => false,
            },
        },
    )
}

impl Oem {
    pub(crate) fn validate_xml_text(&self) -> Result<()> {
        fn check(field: &'static str, value: &str, path: String) -> Result<()> {
            match crate::validation::xml_text_error(field, value) {
                Some(error) => Err(error.at_path(path).into()),
                None => Ok(()),
            }
        }
        fn comments(values: &[String], path: String) -> Result<()> {
            for value in values {
                check("COMMENT", value, path.clone())?;
            }
            Ok(())
        }

        comments(&self.header.comment, "header.comment".into())?;
        if let Some(value) = &self.header.classification {
            check("CLASSIFICATION", value, "header.classification".into())?;
        }
        check(
            "ORIGINATOR",
            &self.header.originator,
            "header.originator".into(),
        )?;
        if let Some(value) = &self.header.message_id {
            check("MESSAGE_ID", value, "header.message_id".into())?;
        }
        for (index, segment) in self.body.segment.iter().enumerate() {
            let base = format!("body.segment[{index}]");
            let metadata = &segment.metadata;
            comments(&metadata.comment, format!("{base}.metadata.comment"))?;
            for (field, value, member) in [
                ("OBJECT_NAME", metadata.object_name.as_str(), "object_name"),
                ("OBJECT_ID", metadata.object_id.as_str(), "object_id"),
                ("CENTER_NAME", metadata.center_name.as_str(), "center_name"),
                ("REF_FRAME", metadata.ref_frame.as_str(), "ref_frame"),
                ("TIME_SYSTEM", metadata.time_system.as_str(), "time_system"),
            ] {
                check(field, value, format!("{base}.metadata.{member}"))?;
            }
            if let Some(value) = &metadata.interpolation {
                check(
                    "INTERPOLATION",
                    value,
                    format!("{base}.metadata.interpolation"),
                )?;
            }
            comments(&segment.data.comment, format!("{base}.data.comment"))?;
            for (covariance_index, covariance) in segment.data.covariance_matrix.iter().enumerate()
            {
                let cov_base = format!("{base}.data.covariance_matrix[{covariance_index}]");
                comments(&covariance.comment, format!("{cov_base}.comment"))?;
                if let Some(value) = &covariance.cov_ref_frame {
                    check("COV_REF_FRAME", value, format!("{cov_base}.cov_ref_frame"))?;
                }
            }
        }
        Ok(())
    }

    /// Strictly parse and validate an OEM XML document.
    pub(crate) fn from_xml_strict(xml: &str) -> Result<Self> {
        let mut source_edition = None;
        (|| {
            validate_envelope(xml, &mut source_edition)?;
            let mut oem: Self = crate::xml::from_str_with_context(xml, "OEM")?;
            oem.normalize_implicit_units();
            crate::traits::Validate::validate(&oem)?;
            Ok(oem)
        })()
        .map_err(|error: crate::error::CcsdsNdmError| {
            error.with_parse_context(
                crate::validation::MessageKind::Oem,
                crate::error::DiagnosticNotation::Xml,
                xml,
                source_edition.as_deref(),
            )
        })
    }

    fn normalize_implicit_units(&mut self) {
        for segment in &mut self.body.segment {
            for state in &mut segment.data.state_vector {
                state.x.units.get_or_insert(PositionUnits::Km);
                state.y.units.get_or_insert(PositionUnits::Km);
                state.z.units.get_or_insert(PositionUnits::Km);
                state.x_dot.units.get_or_insert(VelocityUnits::KmPerS);
                state.y_dot.units.get_or_insert(VelocityUnits::KmPerS);
                state.z_dot.units.get_or_insert(VelocityUnits::KmPerS);
                for acceleration in [&mut state.x_ddot, &mut state.y_ddot, &mut state.z_ddot]
                    .into_iter()
                    .flatten()
                {
                    acceleration.units.get_or_insert(AccUnits::KmPerS2);
                }
            }
            for covariance in &mut segment.data.covariance_matrix {
                for value in [
                    &mut covariance.cx_x,
                    &mut covariance.cy_x,
                    &mut covariance.cy_y,
                    &mut covariance.cz_x,
                    &mut covariance.cz_y,
                    &mut covariance.cz_z,
                ] {
                    value.units.get_or_insert(PositionCovarianceUnits::Km2);
                }
                for value in [
                    &mut covariance.cx_dot_x,
                    &mut covariance.cx_dot_y,
                    &mut covariance.cx_dot_z,
                    &mut covariance.cy_dot_x,
                    &mut covariance.cy_dot_y,
                    &mut covariance.cy_dot_z,
                    &mut covariance.cz_dot_x,
                    &mut covariance.cz_dot_y,
                    &mut covariance.cz_dot_z,
                ] {
                    value
                        .units
                        .get_or_insert(PositionVelocityCovarianceUnits::Km2PerS);
                }
                for value in [
                    &mut covariance.cx_dot_x_dot,
                    &mut covariance.cy_dot_x_dot,
                    &mut covariance.cy_dot_y_dot,
                    &mut covariance.cz_dot_x_dot,
                    &mut covariance.cz_dot_y_dot,
                    &mut covariance.cz_dot_z_dot,
                ] {
                    value.units.get_or_insert(VelocityCovarianceUnits::Km2PerS2);
                }
            }
        }
    }
}
