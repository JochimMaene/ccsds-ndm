// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::{CovLine, ManLine, Ocm, TrajLine};
use crate::error::{Result, ValidationError};
use crate::types::Epoch;
use fast_float;
use serde::{Deserialize, Serialize};

fn ocm_xml_children(parent: &[u8]) -> Option<&'static [&'static [u8]]> {
    Some(match parent {
        b"ocm" => &[b"header", b"body"],
        b"header" => &[
            b"COMMENT",
            b"CLASSIFICATION",
            b"CREATION_DATE",
            b"ORIGINATOR",
            b"MESSAGE_ID",
        ],
        b"body" => &[b"segment"],
        b"segment" => &[b"metadata", b"data"],
        b"metadata" => &[
            b"COMMENT",
            b"OBJECT_NAME",
            b"INTERNATIONAL_DESIGNATOR",
            b"CATALOG_NAME",
            b"OBJECT_DESIGNATOR",
            b"ALTERNATE_NAMES",
            b"ORIGINATOR_POC",
            b"ORIGINATOR_POSITION",
            b"ORIGINATOR_PHONE",
            b"ORIGINATOR_EMAIL",
            b"ORIGINATOR_ADDRESS",
            b"TECH_ORG",
            b"TECH_POC",
            b"TECH_POSITION",
            b"TECH_PHONE",
            b"TECH_EMAIL",
            b"TECH_ADDRESS",
            b"PREVIOUS_MESSAGE_ID",
            b"NEXT_MESSAGE_ID",
            b"ADM_MSG_LINK",
            b"CDM_MSG_LINK",
            b"PRM_MSG_LINK",
            b"RDM_MSG_LINK",
            b"TDM_MSG_LINK",
            b"OPERATOR",
            b"OWNER",
            b"COUNTRY",
            b"CONSTELLATION",
            b"OBJECT_TYPE",
            b"TIME_SYSTEM",
            b"EPOCH_TZERO",
            b"OPS_STATUS",
            b"ORBIT_CATEGORY",
            b"OCM_DATA_ELEMENTS",
            b"SCLK_OFFSET_AT_EPOCH",
            b"SCLK_SEC_PER_SI_SEC",
            b"PREVIOUS_MESSAGE_EPOCH",
            b"NEXT_MESSAGE_EPOCH",
            b"START_TIME",
            b"STOP_TIME",
            b"TIME_SPAN",
            b"TAIMUTC_AT_TZERO",
            b"NEXT_LEAP_EPOCH",
            b"NEXT_LEAP_TAIMUTC",
            b"UT1MUTC_AT_TZERO",
            b"EOP_SOURCE",
            b"INTERP_METHOD_EOP",
            b"CELESTIAL_SOURCE",
        ],
        b"data" => &[b"traj", b"phys", b"cov", b"man", b"pert", b"od", b"user"],
        b"traj" => &[
            b"COMMENT",
            b"TRAJ_ID",
            b"TRAJ_PREV_ID",
            b"TRAJ_NEXT_ID",
            b"TRAJ_BASIS",
            b"TRAJ_BASIS_ID",
            b"INTERPOLATION",
            b"INTERPOLATION_DEGREE",
            b"PROPAGATOR",
            b"CENTER_NAME",
            b"TRAJ_REF_FRAME",
            b"TRAJ_FRAME_EPOCH",
            b"USEABLE_START_TIME",
            b"USEABLE_STOP_TIME",
            b"ORB_REVNUM",
            b"ORB_REVNUM_BASIS",
            b"TRAJ_TYPE",
            b"ORB_AVERAGING",
            b"TRAJ_UNITS",
            b"trajLine",
        ],
        b"phys" => &[
            b"COMMENT",
            b"MANUFACTURER",
            b"BUS_MODEL",
            b"DOCKED_WITH",
            b"DRAG_CONST_AREA",
            b"DRAG_COEFF_NOM",
            b"DRAG_UNCERTAINTY",
            b"INITIAL_WET_MASS",
            b"WET_MASS",
            b"DRY_MASS",
            b"OEB_PARENT_FRAME",
            b"OEB_PARENT_FRAME_EPOCH",
            b"OEB_Q1",
            b"OEB_Q2",
            b"OEB_Q3",
            b"OEB_QC",
            b"OEB_MAX",
            b"OEB_INT",
            b"OEB_MIN",
            b"AREA_ALONG_OEB_MAX",
            b"AREA_ALONG_OEB_INT",
            b"AREA_ALONG_OEB_MIN",
            b"AREA_MIN_FOR_PC",
            b"AREA_MAX_FOR_PC",
            b"AREA_TYP_FOR_PC",
            b"RCS",
            b"RCS_MIN",
            b"RCS_MAX",
            b"SRP_CONST_AREA",
            b"SOLAR_RAD_COEFF",
            b"SOLAR_RAD_UNCERTAINTY",
            b"VM_ABSOLUTE",
            b"VM_APPARENT_MIN",
            b"VM_APPARENT",
            b"VM_APPARENT_MAX",
            b"REFLECTANCE",
            b"ATT_CONTROL_MODE",
            b"ATT_ACTUATOR_TYPE",
            b"ATT_KNOWLEDGE",
            b"ATT_CONTROL",
            b"ATT_POINTING",
            b"AVG_MANEUVER_FREQ",
            b"MAX_THRUST",
            b"DV_BOL",
            b"DV_REMAINING",
            b"IXX",
            b"IYY",
            b"IZZ",
            b"IXY",
            b"IXZ",
            b"IYZ",
        ],
        b"cov" => &[
            b"COMMENT",
            b"COV_ID",
            b"COV_PREV_ID",
            b"COV_NEXT_ID",
            b"COV_BASIS",
            b"COV_BASIS_ID",
            b"COV_REF_FRAME",
            b"COV_FRAME_EPOCH",
            b"COV_SCALE_MIN",
            b"COV_SCALE_MAX",
            b"COV_CONFIDENCE",
            b"COV_TYPE",
            b"COV_ORDERING",
            b"COV_UNITS",
            b"covLine",
        ],
        b"man" => &[
            b"COMMENT",
            b"MAN_ID",
            b"MAN_PREV_ID",
            b"MAN_NEXT_ID",
            b"MAN_BASIS",
            b"MAN_BASIS_ID",
            b"MAN_DEVICE_ID",
            b"MAN_PREV_EPOCH",
            b"MAN_NEXT_EPOCH",
            b"MAN_PURPOSE",
            b"MAN_PRED_SOURCE",
            b"MAN_REF_FRAME",
            b"MAN_FRAME_EPOCH",
            b"GRAV_ASSIST_NAME",
            b"DC_TYPE",
            b"DC_WIN_OPEN",
            b"DC_WIN_CLOSE",
            b"DC_MIN_CYCLES",
            b"DC_MAX_CYCLES",
            b"DC_EXEC_START",
            b"DC_EXEC_STOP",
            b"DC_REF_TIME",
            b"DC_TIME_PULSE_DURATION",
            b"DC_TIME_PULSE_PERIOD",
            b"DC_REF_DIR",
            b"DC_BODY_FRAME",
            b"DC_BODY_TRIGGER",
            b"DC_PA_START_ANGLE",
            b"DC_PA_STOP_ANGLE",
            b"MAN_COMPOSITION",
            b"MAN_UNITS",
            b"manLine",
        ],
        b"pert" => &[
            b"COMMENT",
            b"ATMOSPHERIC_MODEL",
            b"GRAVITY_MODEL",
            b"EQUATORIAL_RADIUS",
            b"GM",
            b"N_BODY_PERTURBATIONS",
            b"CENTRAL_BODY_ROTATION",
            b"OBLATE_FLATTENING",
            b"OCEAN_TIDES_MODEL",
            b"SOLID_TIDES_MODEL",
            b"REDUCTION_THEORY",
            b"ALBEDO_MODEL",
            b"ALBEDO_GRID_SIZE",
            b"SHADOW_MODEL",
            b"SHADOW_BODIES",
            b"SRP_MODEL",
            b"SW_DATA_SOURCE",
            b"SW_DATA_EPOCH",
            b"SW_INTERP_METHOD",
            b"FIXED_GEOMAG_KP",
            b"FIXED_GEOMAG_AP",
            b"FIXED_GEOMAG_DST",
            b"FIXED_F10P7",
            b"FIXED_F10P7_MEAN",
            b"FIXED_M10P7",
            b"FIXED_M10P7_MEAN",
            b"FIXED_S10P7",
            b"FIXED_S10P7_MEAN",
            b"FIXED_Y10P7",
            b"FIXED_Y10P7_MEAN",
        ],
        b"od" => &[
            b"COMMENT",
            b"OD_ID",
            b"OD_PREV_ID",
            b"OD_METHOD",
            b"OD_EPOCH",
            b"DAYS_SINCE_FIRST_OBS",
            b"DAYS_SINCE_LAST_OBS",
            b"RECOMMENDED_OD_SPAN",
            b"ACTUAL_OD_SPAN",
            b"OBS_AVAILABLE",
            b"OBS_USED",
            b"TRACKS_AVAILABLE",
            b"TRACKS_USED",
            b"MAXIMUM_OBS_GAP",
            b"OD_EPOCH_EIGMAJ",
            b"OD_EPOCH_EIGINT",
            b"OD_EPOCH_EIGMIN",
            b"OD_MAX_PRED_EIGMAJ",
            b"OD_MIN_PRED_EIGMIN",
            b"OD_CONFIDENCE",
            b"GDOP",
            b"SOLVE_N",
            b"SOLVE_STATES",
            b"CONSIDER_N",
            b"CONSIDER_PARAMS",
            b"SEDR",
            b"SENSORS_N",
            b"SENSORS",
            b"WEIGHTED_RMS",
            b"DATA_TYPES",
        ],
        b"user" => &[b"COMMENT", b"USER_DEFINED"],
        _ => return None,
    })
}

pub(super) fn validate_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;
    crate::xml::validate_element_sequences(
        xml,
        "OCM",
        |parent, child| {
            let children = ocm_xml_children(parent)?;
            let rank = children.iter().position(|candidate| *candidate == child)? as u16;
            let repeatable = matches!(
                child,
                b"COMMENT"
                    | b"traj"
                    | b"cov"
                    | b"man"
                    | b"trajLine"
                    | b"covLine"
                    | b"manLine"
                    | b"USER_DEFINED"
            );
            // `userDefinedType` wraps its children in a repeating sequence, so a COMMENT may
            // open a new iteration after a USER_DEFINED.
            if parent == b"user" {
                return Some(XmlSequenceRule::restarting(rank, repeatable));
            }
            Some(XmlSequenceRule::new(rank, repeatable))
        },
        |element, attribute| {
            (attribute == b"parameter" && element == b"USER_DEFINED")
                || (attribute == b"units"
                    && matches!(
                        element,
                        b"SCLK_OFFSET_AT_EPOCH"
                            | b"SCLK_SEC_PER_SI_SEC"
                            | b"TIME_SPAN"
                            | b"TAIMUTC_AT_TZERO"
                            | b"NEXT_LEAP_TAIMUTC"
                            | b"UT1MUTC_AT_TZERO"
                            | b"DRAG_CONST_AREA"
                            | b"DRAG_UNCERTAINTY"
                            | b"INITIAL_WET_MASS"
                            | b"WET_MASS"
                            | b"DRY_MASS"
                            | b"OEB_MAX"
                            | b"OEB_INT"
                            | b"OEB_MIN"
                            | b"AREA_ALONG_OEB_MAX"
                            | b"AREA_ALONG_OEB_INT"
                            | b"AREA_ALONG_OEB_MIN"
                            | b"AREA_MIN_FOR_PC"
                            | b"AREA_MAX_FOR_PC"
                            | b"AREA_TYP_FOR_PC"
                            | b"RCS"
                            | b"RCS_MIN"
                            | b"RCS_MAX"
                            | b"SRP_CONST_AREA"
                            | b"SOLAR_RAD_UNCERTAINTY"
                            | b"ATT_KNOWLEDGE"
                            | b"ATT_CONTROL"
                            | b"ATT_POINTING"
                            | b"AVG_MANEUVER_FREQ"
                            | b"MAX_THRUST"
                            | b"DV_BOL"
                            | b"DV_REMAINING"
                            | b"IXX"
                            | b"IYY"
                            | b"IZZ"
                            | b"IXY"
                            | b"IXZ"
                            | b"IYZ"
                            | b"COV_CONFIDENCE"
                            | b"DC_TIME_PULSE_DURATION"
                            | b"DC_TIME_PULSE_PERIOD"
                            | b"DC_PA_START_ANGLE"
                            | b"DC_PA_STOP_ANGLE"
                            | b"EQUATORIAL_RADIUS"
                            | b"GM"
                            | b"CENTRAL_BODY_ROTATION"
                            | b"FIXED_GEOMAG_KP"
                            | b"FIXED_GEOMAG_AP"
                            | b"FIXED_GEOMAG_DST"
                            | b"FIXED_F10P7"
                            | b"FIXED_F10P7_MEAN"
                            | b"FIXED_M10P7"
                            | b"FIXED_M10P7_MEAN"
                            | b"FIXED_S10P7"
                            | b"FIXED_S10P7_MEAN"
                            | b"FIXED_Y10P7"
                            | b"FIXED_Y10P7_MEAN"
                            | b"DAYS_SINCE_FIRST_OBS"
                            | b"DAYS_SINCE_LAST_OBS"
                            | b"RECOMMENDED_OD_SPAN"
                            | b"ACTUAL_OD_SPAN"
                            | b"MAXIMUM_OBS_GAP"
                            | b"OD_EPOCH_EIGMAJ"
                            | b"OD_EPOCH_EIGINT"
                            | b"OD_EPOCH_EIGMIN"
                            | b"OD_MAX_PRED_EIGMAJ"
                            | b"OD_MIN_PRED_EIGMIN"
                            | b"OD_CONFIDENCE"
                            | b"SEDR"
                            | b"WEIGHTED_RMS"
                    ))
        },
    )
}

impl Ocm {
    pub(crate) fn validate_xml_representability(&self) -> Result<()> {
        let data = &self.body.segment.data;
        for (index, man) in data.man.iter().enumerate() {
            for (field, angle) in [
                ("DC_PA_START_ANGLE", man.dc_pa_start_angle.as_ref()),
                ("DC_PA_STOP_ANGLE", man.dc_pa_stop_angle.as_ref()),
            ] {
                let Some(angle) = angle else { continue };
                if !(-360.0..360.0).contains(&angle.value) {
                    return Err(ValidationError::OutOfRange {
                        name: field.into(),
                        value: angle.value.to_string(),
                        expected: "[-360, 360) for the 3.0 XML edition".into(),
                        line: None,
                    }
                    .at_path(format!("body.segment.data.man[{index}]"))
                    .into());
                }
            }
        }
        if let Some(od) = &data.od {
            for (field, value) in [
                ("DAYS_SINCE_FIRST_OBS", &od.days_since_first_obs),
                ("DAYS_SINCE_LAST_OBS", &od.days_since_last_obs),
            ] {
                let Some(value) = value else { continue };
                if value.value < 0.0 {
                    return Err(ValidationError::OutOfRange {
                        name: field.into(),
                        value: value.value.to_string(),
                        expected: ">= 0 for the 3.0 XML edition".into(),
                        line: None,
                    }
                    .at_path("body.segment.data.od")
                    .into());
                }
            }
        }
        Ok(())
    }
}

impl Serialize for TrajLine {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = self.epoch.to_string();
        for v in &self.values {
            s.push(' ');
            s.push_str(&v.to_string());
        }
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for TrajLine {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::utils::deserialize_parsed(
            deserializer,
            "an OCM trajectory line",
            |s| -> std::result::Result<TrajLine, String> {
                let mut parts = s.split_whitespace();
                let epoch = parts
                    .next()
                    .ok_or_else(|| "Missing epoch".to_string())?
                    .parse::<Epoch>()
                    .map_err(|error| error.to_string())?;
                let values: std::result::Result<Vec<f64>, _> = parts
                    .map(|v| fast_float::parse(v).map_err(|error| error.to_string()))
                    .collect();
                Ok(TrajLine {
                    epoch,
                    values: values?,
                })
            },
        )
    }
}

impl Serialize for CovLine {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = self.epoch.to_string();
        for v in &self.values {
            s.push(' ');
            s.push_str(&v.to_string());
        }
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for CovLine {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::utils::deserialize_parsed(
            deserializer,
            "an OCM covariance line",
            |s| -> std::result::Result<CovLine, String> {
                let mut parts = s.split_whitespace();
                let epoch = parts
                    .next()
                    .ok_or_else(|| "Missing epoch".to_string())?
                    .parse::<Epoch>()
                    .map_err(|error| error.to_string())?;
                let values: std::result::Result<Vec<f64>, _> = parts
                    .map(|v| fast_float::parse(v).map_err(|error| error.to_string()))
                    .collect();
                Ok(CovLine {
                    epoch,
                    values: values?,
                })
            },
        )
    }
}

impl Serialize for ManLine {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = self.epoch.to_string();
        for v in &self.values {
            s.push(' ');
            s.push_str(v);
        }
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for ManLine {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::utils::deserialize_parsed(
            deserializer,
            "an OCM maneuver line",
            |s| -> std::result::Result<ManLine, String> {
                let mut parts = s.split_whitespace();
                let epoch = parts
                    .next()
                    .ok_or_else(|| "Missing epoch".to_string())?
                    .parse::<Epoch>()
                    .map_err(|error| error.to_string())?;
                let values: Vec<String> = parts.map(|s| s.to_string()).collect();
                Ok(ManLine { epoch, values })
            },
        )
    }
}
