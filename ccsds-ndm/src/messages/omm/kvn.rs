// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OMM (Orbit Mean-Elements Message).
//!
//! This module implements KVN parsing for OMM using winnow parser combinators.

use super::{MeanElements, Omm, OmmBody, OmmData, OmmMetadata, OmmSegment, TleParameters};
use crate::error::Result;
use crate::kvn::parser::*;
use crate::kvn::ser::{KvnWriter, OdmFloat};
use crate::parse_block;
use crate::traits::ToKvn;
use crate::types::*;
use winnow::prelude::*;

//----------------------------------------------------------------------
// OMM Version Parser
//----------------------------------------------------------------------

pub fn omm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_OMM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// OMM Metadata Parser
//----------------------------------------------------------------------

pub fn omm_metadata(input: &mut &str) -> KvnResult<OmmMetadata> {
    ws.parse_next(input)?;
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut object_id = None;
    let mut center_name = None;
    let mut ref_frame = None;
    let mut ref_frame_epoch = None;
    let mut time_system = None;
    let mut mean_element_theory = None;

    parse_block!(input, comment, {
        "OBJECT_NAME" => object_name: kv_string,
        "OBJECT_ID" => object_id: kv_string,
        "CENTER_NAME" => center_name: kv_string,
        "REF_FRAME" => ref_frame: kv_string,
        "REF_FRAME_EPOCH" => ref_frame_epoch: kv_calendar_epoch,
        "TIME_SYSTEM" => time_system: kv_string,
        "MEAN_ELEMENT_THEORY" => mean_element_theory: kv_string,
    }, |_| false);

    Ok(OmmMetadata {
        comment,
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "OBJECT_NAME"))?,
        object_id: object_id
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "OBJECT_ID"))?,
        center_name: center_name
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "CENTER_NAME"))?,
        ref_frame: ref_frame
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "REF_FRAME"))?,
        ref_frame_epoch,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "TIME_SYSTEM"))?,
        mean_element_theory: mean_element_theory
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "MEAN_ELEMENT_THEORY"))?,
    })
}

//----------------------------------------------------------------------
// Mean Elements Parser
//----------------------------------------------------------------------

pub fn mean_elements(input: &mut &str) -> KvnResult<(Vec<String>, MeanElements)> {
    ws.parse_next(input)?;
    let mut comment = Vec::new();
    let mut epoch = None;
    let mut semi_major_axis = None;
    let mut mean_motion = None;
    let mut eccentricity = None;
    let mut inclination = None;
    let mut ra_of_asc_node = None;
    let mut arg_of_pericenter = None;
    let mut mean_anomaly = None;
    let mut gm = None;

    parse_block!(input, comment, {
        "EPOCH" => epoch: kv_calendar_epoch,
        "SEMI_MAJOR_AXIS" => semi_major_axis: kv_from_kvn,
        "MEAN_MOTION" => mean_motion: kv_from_kvn,
        "ECCENTRICITY" => eccentricity: kv_from_kvn,
        "INCLINATION" => inclination: kv_from_kvn,
        "RA_OF_ASC_NODE" => ra_of_asc_node: kv_from_kvn,
        "ARG_OF_PERICENTER" => arg_of_pericenter: kv_from_kvn,
        "MEAN_ANOMALY" => mean_anomaly: kv_from_kvn,
        "GM" => gm: kv_from_kvn,
    }, |_| false);

    Ok((
        comment,
        MeanElements {
            comment: Vec::new(), // comment is returned as part of the tuple for OmmData
            epoch: epoch.ok_or_else(|| missing_field_err(input, "Mean Elements", "EPOCH"))?,
            semi_major_axis,
            mean_motion,
            eccentricity: eccentricity
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "ECCENTRICITY"))?,
            inclination: inclination
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "INCLINATION"))?,
            ra_of_asc_node: ra_of_asc_node
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "RA_OF_ASC_NODE"))?,
            arg_of_pericenter: arg_of_pericenter
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "ARG_OF_PERICENTER"))?,
            mean_anomaly: mean_anomaly
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "MEAN_ANOMALY"))?,
            gm,
        },
    ))
}

//----------------------------------------------------------------------
// TLE Parameters Parser
//----------------------------------------------------------------------

pub fn tle_parameters(input: &mut &str) -> KvnResult<Option<TleParameters>> {
    ws.parse_next(input)?;
    let mut comment = Vec::new();
    let mut ephemeris_type = None;
    let mut classification_type = None;
    let mut norad_cat_id = None;
    let mut element_set_no = None;
    let mut rev_at_epoch = None;
    let mut bstar = None;
    let mut bterm = None;
    let mut mean_motion_dot = None;
    let mut mean_motion_ddot = None;
    let mut agom = None;

    parse_block!(input, comment, {
        "EPHEMERIS_TYPE" => ephemeris_type: kv_i32,
        "CLASSIFICATION_TYPE" => classification_type: kv_string,
        "NORAD_CAT_ID" => norad_cat_id: kv_u32,
        "ELEMENT_SET_NO" => val: kv_u32 => { element_set_no = Some(val.into()); },
        "REV_AT_EPOCH" => rev_at_epoch: kv_u32,
        "BSTAR" => bstar: kv_from_kvn,
        "BTERM" => bterm: kv_from_kvn,
        "MEAN_MOTION_DOT" => mean_motion_dot: kv_from_kvn,
        "MEAN_MOTION_DDOT" => mean_motion_ddot: kv_from_kvn,
        "AGOM" => agom: kv_from_kvn,
    }, |_| false);

    if ephemeris_type.is_none()
        && classification_type.is_none()
        && norad_cat_id.is_none()
        && element_set_no.is_none()
        && rev_at_epoch.is_none()
        && bstar.is_none()
        && bterm.is_none()
        && mean_motion_dot.is_none()
        && mean_motion_ddot.is_none()
        && agom.is_none()
    {
        return Ok(None);
    }

    Ok(Some(TleParameters {
        comment,
        ephemeris_type,
        classification_type,
        norad_cat_id,
        element_set_no,
        rev_at_epoch,
        bstar,
        bterm,
        mean_motion_dot: mean_motion_dot
            .ok_or_else(|| missing_field_err(input, "TLE Parameters", "MEAN_MOTION_DOT"))?,
        mean_motion_ddot,
        agom,
    }))
}

//----------------------------------------------------------------------
// OMM Data Parser
//----------------------------------------------------------------------

pub fn omm_data(input: &mut &str) -> KvnResult<OmmData> {
    let (me_comment, mean_elements) = mean_elements.parse_next(input)?;

    // Spacecraft parameters
    let spacecraft_parameters = spacecraft_parameters.parse_next(input)?;

    // TLE parameters
    let tle_parameters = tle_parameters.parse_next(input)?;

    // Covariance matrix
    let covariance_matrix = covariance_matrix.parse_next(input)?;

    // User defined
    let user_defined_parameters = user_defined_parameters.parse_next(input)?;

    Ok(OmmData {
        comment: me_comment,
        mean_elements,
        spacecraft_parameters,
        tle_parameters,
        covariance_matrix,
        user_defined_parameters,
    })
}

//----------------------------------------------------------------------
// Complete OMM Parser
//----------------------------------------------------------------------

pub fn parse_omm(input: &mut &str) -> KvnResult<Omm> {
    let version = omm_version.parse_next(input)?;
    let header = odm_header.parse_next(input)?;
    let metadata = omm_metadata.parse_next(input)?;
    let data = omm_data.parse_next(input)?;

    Ok(Omm {
        header,
        body: OmmBody {
            segment: OmmSegment { metadata, data },
        },
        id: Some("CCSDS_OMM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Omm {
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_omm.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

pub(super) fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    fn rank(key: &str) -> Option<u16> {
        Some(match key {
            "CCSDS_OMM_VERS" => 0,
            "CLASSIFICATION" => 1,
            "CREATION_DATE" => 2,
            "ORIGINATOR" => 3,
            "MESSAGE_ID" => 4,
            "OBJECT_NAME" => 5,
            "OBJECT_ID" => 6,
            "CENTER_NAME" => 7,
            "REF_FRAME" => 8,
            "REF_FRAME_EPOCH" => 9,
            "TIME_SYSTEM" => 10,
            "MEAN_ELEMENT_THEORY" => 11,
            "EPOCH" => 12,
            "SEMI_MAJOR_AXIS" | "MEAN_MOTION" => 13,
            "ECCENTRICITY" => 14,
            "INCLINATION" => 15,
            "RA_OF_ASC_NODE" => 16,
            "ARG_OF_PERICENTER" => 17,
            "MEAN_ANOMALY" => 18,
            "GM" => 19,
            "MASS" => 30,
            "SOLAR_RAD_AREA" => 31,
            "SOLAR_RAD_COEFF" => 32,
            "DRAG_AREA" => 33,
            "DRAG_COEFF" => 34,
            "EPHEMERIS_TYPE" => 40,
            "CLASSIFICATION_TYPE" => 41,
            "NORAD_CAT_ID" => 42,
            "ELEMENT_SET_NO" => 43,
            "REV_AT_EPOCH" => 44,
            "BSTAR" | "BTERM" => 45,
            "MEAN_MOTION_DOT" => 46,
            "MEAN_MOTION_DDOT" | "AGOM" => 47,
            "COV_REF_FRAME" => 60,
            "CX_X" => 61,
            "CY_X" => 62,
            "CY_Y" => 63,
            "CZ_X" => 64,
            "CZ_Y" => 65,
            "CZ_Z" => 66,
            "CX_DOT_X" => 67,
            "CX_DOT_Y" => 68,
            "CX_DOT_Z" => 69,
            "CX_DOT_X_DOT" => 70,
            "CY_DOT_X" => 71,
            "CY_DOT_Y" => 72,
            "CY_DOT_Z" => 73,
            "CY_DOT_X_DOT" => 74,
            "CY_DOT_Y_DOT" => 75,
            "CZ_DOT_X" => 76,
            "CZ_DOT_Y" => 77,
            "CZ_DOT_Z" => 78,
            "CZ_DOT_X_DOT" => 79,
            "CZ_DOT_Y_DOT" => 80,
            "CZ_DOT_Z_DOT" => 81,
            key if key.starts_with("USER_DEFINED_") => 90,
            _ => return None,
        })
    }

    fn comments_start_block(previous: u16, key: &str) -> bool {
        match key {
            "CLASSIFICATION" | "CREATION_DATE" => previous == 0,
            "OBJECT_NAME" => matches!(previous, 3 | 4),
            "EPOCH" => previous == 11,
            "MASS" | "SOLAR_RAD_AREA" | "SOLAR_RAD_COEFF" | "DRAG_AREA" | "DRAG_COEFF" => {
                matches!(previous, 18 | 19)
            }
            "EPHEMERIS_TYPE"
            | "CLASSIFICATION_TYPE"
            | "NORAD_CAT_ID"
            | "ELEMENT_SET_NO"
            | "REV_AT_EPOCH"
            | "BSTAR"
            | "BTERM"
            | "MEAN_MOTION_DOT"
            | "MEAN_MOTION_DDOT"
            | "AGOM" => matches!(previous, 18 | 19 | 30..=34),
            "COV_REF_FRAME" | "CX_X" => {
                matches!(previous, 18 | 19 | 30..=34 | 40..=47)
            }
            key if key.starts_with("USER_DEFINED_") => {
                matches!(previous, 18 | 19 | 30..=34 | 40..=47 | 61..=81)
            }
            _ => false,
        }
    }

    crate::kvn::strict::validate_odm_assignments(
        kvn,
        &crate::kvn::strict::OdmAssignmentRules {
            context: "strict OMM KVN",
            message_name: "OMM",
            rank,
            comment_starts_block: comments_start_block,
            allows_non_increasing: |previous, current| {
                // SEMI_MAJOR_AXIS/MEAN_MOTION, BSTAR/BTERM, and MEAN_MOTION_DDOT/AGOM each share
                // a rank so either spelling may fill the slot; only the *other* alternative may
                // follow, never a repeat of the same keyword. USER_DEFINED_* genuinely repeats.
                (matches!(previous.rank, 13 | 45 | 47)
                    && current.rank == previous.rank
                    && current.key != previous.key)
                    || (current.rank == 90 && previous.rank == 90)
            },
        },
    )
}

impl ToKvn for Omm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // 1. Header
        writer.write_pair("CCSDS_OMM_VERS", &self.version);
        self.header.write_kvn(writer);

        // 2. Body
        self.body.write_kvn(writer);
    }
}

impl ToKvn for OmmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl ToKvn for OmmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl ToKvn for OmmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("OBJECT_ID", &self.object_id);
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("REF_FRAME", &self.ref_frame);
        if let Some(v) = &self.ref_frame_epoch {
            writer.write_pair("REF_FRAME_EPOCH", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("MEAN_ELEMENT_THEORY", &self.mean_element_theory);
    }
}

impl ToKvn for OmmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // Mean Elements
        self.mean_elements.write_kvn(writer);

        // Spacecraft Params
        if let Some(sp) = &self.spacecraft_parameters {
            writer.write_comments(&sp.comment);
            if let Some(v) = &sp.mass {
                writer.write_odm_float_measure("MASS", &v.to_unit_value());
            }
            if let Some(v) = &sp.solar_rad_area {
                writer.write_odm_float_measure("SOLAR_RAD_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.solar_rad_coeff {
                writer.write_odm_float_pair("SOLAR_RAD_COEFF", v.value);
            }
            if let Some(v) = &sp.drag_area {
                writer.write_odm_float_measure("DRAG_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.drag_coeff {
                writer.write_odm_float_pair("DRAG_COEFF", v.value);
            }
        }

        // TLE Params
        if let Some(tle) = &self.tle_parameters {
            tle.write_kvn(writer);
        }

        // Covariance
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }

        // User Defined
        if let Some(ud) = &self.user_defined_parameters {
            writer.write_comments(&ud.comment);
            for p in &ud.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
        }
    }
}

impl ToKvn for MeanElements {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", self.epoch);
        if let Some(v) = &self.semi_major_axis {
            writer.write_odm_float_measure("SEMI_MAJOR_AXIS", v);
        }
        if let Some(v) = &self.mean_motion {
            writer.write_odm_float_measure("MEAN_MOTION", v);
        }
        writer.write_odm_float_pair("ECCENTRICITY", self.eccentricity.value);
        writer.write_odm_float_measure("INCLINATION", &self.inclination.to_unit_value());
        writer.write_odm_float_measure("RA_OF_ASC_NODE", &self.ra_of_asc_node.to_unit_value());
        writer
            .write_odm_float_measure("ARG_OF_PERICENTER", &self.arg_of_pericenter.to_unit_value());
        writer.write_odm_float_measure("MEAN_ANOMALY", &self.mean_anomaly.to_unit_value());
        if let Some(v) = &self.gm {
            writer.write_odm_float_measure("GM", &UnitValue::new(v.value, v.units.clone()));
        }
    }
}

impl ToKvn for TleParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(v) = self.ephemeris_type {
            writer.write_pair("EPHEMERIS_TYPE", v);
        }
        if let Some(v) = &self.classification_type {
            writer.write_pair("CLASSIFICATION_TYPE", v);
        }
        if let Some(v) = self.norad_cat_id {
            writer.write_pair("NORAD_CAT_ID", v);
        }
        if let Some(v) = self.element_set_no {
            writer.write_pair("ELEMENT_SET_NO", v);
        }
        if let Some(v) = self.rev_at_epoch {
            writer.write_pair("REV_AT_EPOCH", v);
        }
        if let Some(v) = &self.bstar {
            writer.write_odm_float_measure("BSTAR", v);
        }
        if let Some(v) = &self.bterm {
            writer.write_odm_float_measure("BTERM", v);
        }
        writer.write_odm_float_measure("MEAN_MOTION_DOT", &self.mean_motion_dot);
        if let Some(v) = &self.mean_motion_ddot {
            writer.write_odm_float_measure("MEAN_MOTION_DDOT", v);
        }
        if let Some(v) = &self.agom {
            writer.write_odm_float_measure("AGOM", v);
        }
    }
}

impl Omm {
    /// Reject values that KVN generation could not spell back to the same number.
    ///
    /// The schema types these fields as plain doubles, so range and finiteness checks let through
    /// magnitudes whose shortest round-tripping spelling exceeds the 16 significant digits and
    /// 255-character line that ODM 7.7.1 allows. Catching them here keeps generation from
    /// emitting a document this library would refuse to read back.
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        fn check(field: &'static str, value: f64, path: &'static str) -> Result<()> {
            if OdmFloat::is_valid(value) {
                return Ok(());
            }
            Err(crate::validation::unrepresentable_number(
                field, value, path,
            ))
        }

        let data = &self.body.segment.data;
        let elements = &data.mean_elements;
        check(
            "ECCENTRICITY",
            elements.eccentricity.value,
            "body.segment.data.mean_elements.eccentricity",
        )?;
        check(
            "INCLINATION",
            elements.inclination.angle.value,
            "body.segment.data.mean_elements.inclination",
        )?;
        check(
            "RA_OF_ASC_NODE",
            elements.ra_of_asc_node.value,
            "body.segment.data.mean_elements.ra_of_asc_node",
        )?;
        check(
            "ARG_OF_PERICENTER",
            elements.arg_of_pericenter.value,
            "body.segment.data.mean_elements.arg_of_pericenter",
        )?;
        check(
            "MEAN_ANOMALY",
            elements.mean_anomaly.value,
            "body.segment.data.mean_elements.mean_anomaly",
        )?;
        for (field, value, path) in [
            (
                "SEMI_MAJOR_AXIS",
                elements.semi_major_axis.as_ref().map(|v| v.value),
                "body.segment.data.mean_elements.semi_major_axis",
            ),
            (
                "MEAN_MOTION",
                elements.mean_motion.as_ref().map(|v| v.value),
                "body.segment.data.mean_elements.mean_motion",
            ),
            (
                "GM",
                elements.gm.as_ref().map(|v| v.value),
                "body.segment.data.mean_elements.gm",
            ),
        ] {
            let Some(value) = value else { continue };
            check(field, value, path)?;
        }
        if let Some(parameters) = &data.spacecraft_parameters {
            for (field, value, path) in [
                (
                    "MASS",
                    parameters.mass.as_ref().map(|v| v.value),
                    "body.segment.data.spacecraft_parameters.mass",
                ),
                (
                    "SOLAR_RAD_AREA",
                    parameters.solar_rad_area.as_ref().map(|v| v.value),
                    "body.segment.data.spacecraft_parameters.solar_rad_area",
                ),
                (
                    "SOLAR_RAD_COEFF",
                    parameters.solar_rad_coeff.as_ref().map(|v| v.value),
                    "body.segment.data.spacecraft_parameters.solar_rad_coeff",
                ),
                (
                    "DRAG_AREA",
                    parameters.drag_area.as_ref().map(|v| v.value),
                    "body.segment.data.spacecraft_parameters.drag_area",
                ),
                (
                    "DRAG_COEFF",
                    parameters.drag_coeff.as_ref().map(|v| v.value),
                    "body.segment.data.spacecraft_parameters.drag_coeff",
                ),
            ] {
                let Some(value) = value else { continue };
                check(field, value, path)?;
            }
        }

        if let Some(tle) = &data.tle_parameters {
            for (field, value, path) in [
                (
                    "BSTAR",
                    tle.bstar.as_ref().map(|v| v.value),
                    "body.segment.data.tle_parameters.bstar",
                ),
                (
                    "BTERM",
                    tle.bterm.as_ref().map(|v| v.value),
                    "body.segment.data.tle_parameters.bterm",
                ),
                (
                    "MEAN_MOTION_DOT",
                    Some(tle.mean_motion_dot.value),
                    "body.segment.data.tle_parameters.mean_motion_dot",
                ),
                (
                    "MEAN_MOTION_DDOT",
                    tle.mean_motion_ddot.as_ref().map(|v| v.value),
                    "body.segment.data.tle_parameters.mean_motion_ddot",
                ),
                (
                    "AGOM",
                    tle.agom.as_ref().map(|v| v.value),
                    "body.segment.data.tle_parameters.agom",
                ),
            ] {
                let Some(value) = value else { continue };
                check(field, value, path)?;
            }
        }

        if let Some(covariance) = &data.covariance_matrix {
            for (field, value, path) in covariance.kvn_numbers() {
                check(field, value, path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Ndm;

    const MINIMAL_OMM: &str = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2000-06-28T11:59:28.000000
MEAN_MOTION = 1.00273272 [rev/day]
ECCENTRICITY = 0.00050130
INCLINATION = 3.053900 [deg]
RA_OF_ASC_NODE = 81.793900 [deg]
ARG_OF_PERICENTER = 249.236300 [deg]
MEAN_ANOMALY = 150.160200 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;

    #[test]
    fn test_parse_minimal_omm() {
        let result = Omm::from_kvn_str(MINIMAL_OMM);
        assert!(
            result.is_ok(),
            "Failed to parse minimal OMM: {:?}",
            result.err()
        );

        let omm = result.unwrap();
        assert_eq!(omm.version, "3.0");
        assert_eq!(omm.header.originator, "JAXA");
        assert_eq!(omm.body.segment.metadata.object_name, "GOES 9");
    }

    #[test]
    fn test_parse_full_omm() {
        let full_omm = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
REF_FRAME_EPOCH = 2000-06-28T11:59:28
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2000-06-28T11:59:28.000000
SEMI_MAJOR_AXIS = 42164.0 [km]
ECCENTRICITY = 0.00050130
INCLINATION = 3.053900 [deg]
RA_OF_ASC_NODE = 81.793900 [deg]
ARG_OF_PERICENTER = 249.236300 [deg]
MEAN_ANOMALY = 150.160200 [deg]
GM = 398600.4415 [km**3/s**2]
MASS = 1000 [kg]
SOLAR_RAD_AREA = 10 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 5 [m**2]
DRAG_COEFF = 2.2
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 23581
ELEMENT_SET_NO = 999
REV_AT_EPOCH = 1234
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
COV_REF_FRAME = TEME
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.1
CZ_Y = 0.1
CZ_Z = 1.0
CX_DOT_X = 0.1
CX_DOT_Y = 0.1
CX_DOT_Z = 0.1
CX_DOT_X_DOT = 1.0
CY_DOT_X = 0.1
CY_DOT_Y = 0.1
CY_DOT_Z = 0.1
CY_DOT_X_DOT = 0.1
CY_DOT_Y_DOT = 1.0
CZ_DOT_X = 0.1
CZ_DOT_Y = 0.1
CZ_DOT_Z = 0.1
CZ_DOT_X_DOT = 0.1
CZ_DOT_Y_DOT = 0.1
CZ_DOT_Z_DOT = 1.0
USER_DEFINED_FOO = BAR
"#;
        let result = Omm::from_kvn_str(full_omm);
        assert!(
            result.is_ok(),
            "Failed to parse full OMM: {:?}",
            result.err()
        );
        let omm = result.unwrap();
        assert!(omm.body.segment.metadata.ref_frame_epoch.is_some());
        assert!(omm.body.segment.data.spacecraft_parameters.is_some());
        assert!(omm.body.segment.data.tle_parameters.is_some());
        assert!(omm.body.segment.data.covariance_matrix.is_some());
        assert!(omm.body.segment.data.user_defined_parameters.is_some());
    }

    #[test]
    fn frame_reference_epoch_requires_calendar_form() {
        let valid = MINIMAL_OMM.replace(
            "REF_FRAME = TEME\n",
            "REF_FRAME = TEME\nREF_FRAME_EPOCH = 2000-01-01T12:00:00\n",
        );
        let omm = Omm::from_kvn_str(&valid).expect("calendar frame epoch should parse");
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

        let numeric = valid.replace(
            "REF_FRAME_EPOCH = 2000-01-01T12:00:00",
            "REF_FRAME_EPOCH = 123.5",
        );
        assert!(Omm::from_kvn_str(&numeric).is_err());

        let numeric_mean_epoch =
            valid.replace("EPOCH = 2000-06-28T11:59:28.000000", "EPOCH = 123.5");
        assert!(Omm::from_kvn_str(&numeric_mean_epoch).is_err());
    }

    #[test]
    fn test_omm_tle_no_cov() {
        let tle_omm = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2000-06-28T11:59:28.000000
MEAN_MOTION = 1.00273272 [rev/day]
ECCENTRICITY = 0.00050130
INCLINATION = 3.053900 [deg]
RA_OF_ASC_NODE = 81.793900 [deg]
ARG_OF_PERICENTER = 249.236300 [deg]
MEAN_ANOMALY = 150.160200 [deg]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
BSTAR = 0.0001 [1/ER]
AGOM = 0.0001 [m**2/kg]
"#;
        let result = Omm::from_kvn_str(tle_omm);
        assert!(
            result.is_ok(),
            "Failed to parse TLE OMM: {:?}",
            result.err()
        );
        let omm = result.unwrap();
        assert!(omm.body.segment.data.tle_parameters.is_some());
        assert!(omm
            .body
            .segment
            .data
            .tle_parameters
            .as_ref()
            .unwrap()
            .agom
            .is_some());
    }

    #[test]
    fn test_omm_errors() {
        // Unknown metadata key
        let mut input = "OBJECT_NAME = GOES 9\nUNKNOWN_KEY = VAL\n";
        assert!(omm_metadata.parse_next(&mut input).is_err());

        // Exhaustive missing mandatory fields in metadata
        let mandatory_meta = [
            "OBJECT_NAME = GOES 9\n",
            "OBJECT_ID = 1\n",
            "CENTER_NAME = EARTH\n",
            "REF_FRAME = TEME\n",
            "TIME_SYSTEM = UTC\n",
            "MEAN_ELEMENT_THEORY = SGP4\n",
        ];
        for i in 0..mandatory_meta.len() {
            let mut input_str = String::new();
            for (j, item) in mandatory_meta.iter().enumerate() {
                if i != j {
                    input_str.push_str(item);
                }
            }
            let mut input = input_str.as_str();
            assert!(
                omm_metadata.parse_next(&mut input).is_err(),
                "Should fail without {}",
                mandatory_meta[i]
            );
        }

        // Both SEMI_MAJOR_AXIS and MEAN_MOTION parse; semantic check is in validation
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nMEAN_MOTION = 1.0\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0\nMEAN_ANOMALY = 0\n";
        let (_, me_both) = mean_elements.parse_next(&mut input).unwrap();
        assert!(me_both.semi_major_axis.is_some());
        assert!(me_both.mean_motion.is_some());

        // Neither SEMI_MAJOR_AXIS nor MEAN_MOTION parses; semantic check is in validation
        let mut input = "EPOCH = 2000-06-28T11:59:28\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0\nMEAN_ANOMALY = 0\n";
        let (_, me_none) = mean_elements.parse_next(&mut input).unwrap();
        assert!(me_none.semi_major_axis.is_none());
        assert!(me_none.mean_motion.is_none());

        // Negative eccentricity
        let mut input =
            "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = -0.1\n";
        assert!(mean_elements.parse_next(&mut input).is_err());

        // Exhaustive missing mandatory fields in mean elements
        let mandatory_me = [
            "EPOCH = 2000-06-28T11:59:28\n",
            "ECCENTRICITY = 0.1\n",
            "INCLINATION = 0\n",
            "RA_OF_ASC_NODE = 0\n",
            "ARG_OF_PERICENTER = 0\n",
            "MEAN_ANOMALY = 0\n",
        ];
        for i in 0..mandatory_me.len() {
            let mut input_str = String::from("SEMI_MAJOR_AXIS = 42164\n");
            for (j, item) in mandatory_me.iter().enumerate() {
                if i != j {
                    input_str.push_str(item);
                }
            }
            let mut input = input_str.as_str();
            assert!(
                mean_elements.parse_next(&mut input).is_err(),
                "Should fail without {}",
                mandatory_me[i]
            );
        }

        // TLE: both BSTAR and BTERM parses; semantic check is in validation
        let mut input =
            "MEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0\nBSTAR = 0.0001\nBTERM = 0.0001\n";
        let tle = tle_parameters.parse_next(&mut input).unwrap().unwrap();
        assert!(tle.bstar.is_some());
        assert!(tle.bterm.is_some());

        // TLE: DDOT + AGOM parses; semantic check is in validation
        let mut input = "MEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0\nAGOM = 0.0001\n";
        let tle = tle_parameters.parse_next(&mut input).unwrap().unwrap();
        assert!(tle.mean_motion_ddot.is_some());
        assert!(tle.agom.is_some());

        // TLE: independent checks for format

        // TLE: missing MEAN_MOTION_DOT (Mandatory in parser now)
        let mut input = "BSTAR = 0.0001\nMEAN_MOTION_DDOT = 0.0\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // TLE: both MEAN_MOTION_DDOT and AGOM parses; semantic check is in validation
        let mut input =
            "MEAN_MOTION_DOT = 0.000001\nBSTAR = 0.0001\nMEAN_MOTION_DDOT = 0.0\nAGOM = 0.0001\n";
        let tle = tle_parameters.parse_next(&mut input).unwrap().unwrap();
        assert!(tle.mean_motion_ddot.is_some());
        assert!(tle.agom.is_some());

        // TLE: neither MEAN_MOTION_DDOT nor AGOM (Allowed in parser)
        let mut input = "MEAN_MOTION_DOT = 0.000001\nBSTAR = 0.0001\n";
        assert!(tle_parameters.parse_next(&mut input).is_ok());

        // TLE: invalid ELEMENT_SET_NO
        // Now allowed in parser (no range check)
        let mut input = "MEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0\nBSTAR = 0.0001\nELEMENT_SET_NO = 10000\n";
        assert!(tle_parameters.parse_next(&mut input).is_ok());

        // Invalid units for coverage (Existing tests...)
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nMEAN_MOTION = 1.0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input =
            "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = INVALID\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0\nMEAN_ANOMALY = 0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0\nMEAN_ANOMALY = 0\nGM = 398600 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0\nMEAN_ANOMALY = 0\nGM = -1\n";
        assert!(mean_elements.parse_next(&mut input).is_err());

        // TLE invalid formats
        let mut input = "EPHEMERIS_TYPE = INVALID\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "NORAD_CAT_ID = INVALID\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "ELEMENT_SET_NO = INVALID\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "REV_AT_EPOCH = INVALID\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "BSTAR = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "BTERM = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "MEAN_MOTION_DOT = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "MEAN_MOTION_DDOT = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "AGOM = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // Extra error coverage
        let mut input = "REV_AT_EPOCH = 1\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        let mut input = "MEAN_MOTION_DOT = 0\nBSTAR = 0\nREV_AT_EPOCH = 1\nMEAN_MOTION_DDOT = 0\n";
        assert!(tle_parameters.parse_next(&mut input).is_ok());

        let mut input = "OBJECT_NAME = GOES 9\nREF_FRAME_EPOCH = INVALID\n";
        assert!(omm_metadata.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_omm_validation() {
        // Construct a parsed OMM-like structure or just parse minimal incomplete one and validate
        // SGP4 Theory requires BSTAR
        let kvn_sgp4_missing_bstar = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let res = Omm::from_kvn(kvn_sgp4_missing_bstar);
        assert!(res.is_err());
        let err = res.err().unwrap();
        if !err.is_validation_error() {
            panic!("Expected validation error, got: {:?}", err);
        }
        assert!(err.is_validation_error());

        // SGP4 with params but missing BSTAR
        let kvn_sgp4_missing_bstar_field = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
COMMENT TLE Params present but partial
MEAN_MOTION_DOT = 0.001 [rev/day**2]
"#;
        let res = Omm::from_kvn(kvn_sgp4_missing_bstar_field);
        assert!(res.is_err());
        // Should error about missing BSTAR
        assert!(format!("{}", res.err().unwrap()).contains("BSTAR"));

        // Valid SGP4
        let kvn_sgp4_valid = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001
MEAN_MOTION_DOT = 0.0001
"#;
        assert!(Omm::from_kvn(kvn_sgp4_valid).is_ok());
    }

    // =========================================================================
    // Migrated Tests from messages/omm.rs
    // =========================================================================

    #[test]
    fn parse_omm_with_covariance_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
CX_X = 1.0 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 0.01 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 0.01 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 0.01 [km**2/s**2]
"#;
        let omm = Omm::from_kvn(kvn).expect("OMM Covariance parse failed");
        assert!(omm.body.segment.data.covariance_matrix.is_some());
    }

    #[test]
    fn test_mean_elements_choice_semi_major_axis_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with SEMI_MAJOR_AXIS");
        assert!(omm
            .body
            .segment
            .data
            .mean_elements
            .semi_major_axis
            .is_some());
    }

    #[test]
    fn test_mean_elements_choice_mean_motion_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION");
        assert!(omm.body.segment.data.mean_elements.mean_motion.is_some());
    }

    #[test]
    fn test_tle_choice_bstar_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with BSTAR");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.bstar.is_some());
        assert!(tle.bterm.is_none());
    }

    #[test]
    fn test_tle_choice_bterm_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4-XP
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
AGOM = 0.01 [m**2/kg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with BTERM");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.bterm.is_some());
        assert!(tle.bstar.is_none());
    }

    #[test]
    fn test_tle_choice_mean_motion_ddot_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION_DDOT");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.mean_motion_ddot.is_some());
        assert!(tle.agom.is_none());
    }

    #[test]
    fn test_tle_choice_agom_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4-XP
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
AGOM = 0.01 [m**2/kg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with AGOM");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.agom.is_some());
        assert!(tle.mean_motion_ddot.is_none());
    }
}
