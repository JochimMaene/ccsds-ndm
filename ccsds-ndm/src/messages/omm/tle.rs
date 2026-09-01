// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::*;

impl Omm {
    /// Validates the OMM against CCSDS constraints that cannot be checked during parsing.
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
        self.body.segment.validate()
    }

    /// Generate canonical NORAD TLE line 1 and line 2 from this OMM.
    ///
    /// This method requires:
    /// - `MEAN_ELEMENT_THEORY` to be `SGP` or `SGP4`
    /// - presence of `TLE_PARAMETERS`
    /// - either a launch designator in `OBJECT_ID` (`YYYY-NNN[PPP]`) or `OBJECT_ID=UNKNOWN`
    /// - strict field-width compliance for NORAD fixed-width line formats
    pub fn to_tle_lines(&self) -> Result<(String, String)> {
        self.validate()?;

        let metadata = &self.body.segment.metadata;
        let data = &self.body.segment.data;
        let tle = data
            .tle_parameters
            .as_ref()
            .ok_or(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("OMM Data"),
                field: Cow::Borrowed("TLE_PARAMETERS"),
                line: None,
            })?;

        let theory = metadata.mean_element_theory.trim();
        if !matches!(theory, "SGP" | "SGP4" | "SGP/SGP4") {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed("MEAN_ELEMENT_THEORY"),
                value: theory.to_string(),
                expected: Cow::Borrowed("SGP, SGP4, or SGP/SGP4"),
                line: None,
            }
            .into());
        }

        if metadata.time_system.trim().to_uppercase() != "UTC" {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed("TIME_SYSTEM"),
                value: metadata.time_system.clone(),
                expected: Cow::Borrowed("UTC"),
                line: None,
            }
            .into());
        }

        let launch = parse_object_id_launch_designator(&metadata.object_id)?;
        let norad_cat_id = require_tle_field(tle.norad_cat_id, "NORAD_CAT_ID")?;
        let norad_cat_id_field = format_tle_satellite_number(norad_cat_id)?;

        let classification =
            tle.classification_type
                .as_ref()
                .ok_or(ValidationError::MissingRequiredField {
                    block: Cow::Borrowed("TLE Parameters"),
                    field: Cow::Borrowed("CLASSIFICATION_TYPE"),
                    line: None,
                })?;
        let classification_char = parse_classification_char(classification)?;

        let ephemeris_type = require_tle_field(tle.ephemeris_type, "EPHEMERIS_TYPE")?;
        if !(0..=9).contains(&ephemeris_type) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("EPHEMERIS_TYPE"),
                value: ephemeris_type.to_string(),
                expected: Cow::Borrowed("[0, 9]"),
                line: None,
            }
            .into());
        }

        let element_set_no = require_tle_field(tle.element_set_no, "ELEMENT_SET_NO")?;
        let rev_at_epoch = require_tle_field(tle.rev_at_epoch, "REV_AT_EPOCH")?;
        if rev_at_epoch > 99_999 {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("REV_AT_EPOCH"),
                value: rev_at_epoch.to_string(),
                expected: Cow::Borrowed("[0, 99999]"),
                line: None,
            }
            .into());
        }

        let mean_motion = data.mean_elements.mean_motion.as_ref().ok_or(
            ValidationError::MissingRequiredField {
                block: Cow::Borrowed("Mean Elements"),
                field: Cow::Borrowed("MEAN_MOTION"),
                line: None,
            },
        )?;

        let mean_motion_ddot =
            tle.mean_motion_ddot
                .as_ref()
                .ok_or(ValidationError::MissingRequiredField {
                    block: Cow::Borrowed("TLE Parameters"),
                    field: Cow::Borrowed("MEAN_MOTION_DDOT"),
                    line: None,
                })?;

        let bstar = tle
            .bstar
            .as_ref()
            .ok_or(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("TLE Parameters"),
                field: Cow::Borrowed("BSTAR"),
                line: None,
            })?;

        let (epoch_year_2, epoch_day_field) =
            format_tle_epoch_components(data.mean_elements.epoch.as_str())?;
        let mean_motion_dot_field = format_tle_dot_term(tle.mean_motion_dot.value)?;
        let mean_motion_ddot_field = format_tle_assumed_decimal(mean_motion_ddot.value)?;
        let bstar_field = format_tle_assumed_decimal(bstar.value)?;

        let inclination = data.mean_elements.inclination.angle.value;
        let raan = data.mean_elements.ra_of_asc_node.value;
        let arg_pericenter = data.mean_elements.arg_of_pericenter.value;
        let mean_anomaly = data.mean_elements.mean_anomaly.value;
        let eccentricity = data.mean_elements.eccentricity.value;

        validate_angle_for_tle("INCLINATION", inclination, false)?;
        validate_angle_for_tle("RA_OF_ASC_NODE", raan, true)?;
        validate_angle_for_tle("ARG_OF_PERICENTER", arg_pericenter, true)?;
        validate_angle_for_tle("MEAN_ANOMALY", mean_anomaly, true)?;
        if !(0.0..1.0).contains(&eccentricity) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("ECCENTRICITY"),
                value: eccentricity.to_string(),
                expected: Cow::Borrowed("[0.0, 1.0)"),
                line: None,
            }
            .into());
        }
        if !(0.0..100.0).contains(&mean_motion.value) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("MEAN_MOTION"),
                value: mean_motion.value.to_string(),
                expected: Cow::Borrowed("[0.0, 100.0)"),
                line: None,
            }
            .into());
        }

        let ecc_scaled = (eccentricity * 1.0e7).round();
        if !(0.0..10_000_000.0).contains(&ecc_scaled) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("ECCENTRICITY"),
                value: eccentricity.to_string(),
                expected: Cow::Borrowed("encodable as 7 TLE digits"),
                line: None,
            }
            .into());
        }

        let mut line1_no_checksum = format!(
            "1 {}{} {}{}{} {:02}{} {} {} {} {} {:>4}",
            norad_cat_id_field,
            classification_char,
            launch
                .as_ref()
                .map(|v| format!("{:02}", v.launch_year % 100))
                .unwrap_or_else(|| "  ".to_string()),
            launch
                .as_ref()
                .map(|v| format!("{:03}", v.launch_number))
                .unwrap_or_else(|| "   ".to_string()),
            launch
                .as_ref()
                .map(|v| format!("{:<3}", v.launch_piece))
                .unwrap_or_else(|| "   ".to_string()),
            epoch_year_2,
            epoch_day_field,
            mean_motion_dot_field,
            mean_motion_ddot_field,
            bstar_field,
            ephemeris_type,
            element_set_no.value
        );
        line1_no_checksum = normalize_tle_line_len(line1_no_checksum, "line 1")?;
        let checksum1 = tle_checksum(&line1_no_checksum);
        let line1 = format!("{}{}", line1_no_checksum, checksum1);

        let mut line2_no_checksum = format!(
            "2 {} {:8.4} {:8.4} {:07} {:8.4} {:8.4} {:11.8}{:5}",
            norad_cat_id_field,
            inclination,
            raan,
            ecc_scaled as u32,
            arg_pericenter,
            mean_anomaly,
            mean_motion.value,
            rev_at_epoch
        );
        line2_no_checksum = normalize_tle_line_len(line2_no_checksum, "line 2")?;
        let checksum2 = tle_checksum(&line2_no_checksum);
        let line2 = format!("{}{}", line2_no_checksum, checksum2);

        Ok((line1, line2))
    }

    /// Parse NORAD TLE line 1/2 into a minimal OMM.
    pub fn from_tle_lines(line1: &str, line2: &str) -> Result<Self> {
        Self::from_tle_lines_with_options(line1, line2, &TleToOmmOptions::default())
    }

    /// Parse NORAD TLE line 1/2 into a minimal OMM with metadata/header overrides.
    pub fn from_tle_lines_with_options(
        line1: &str,
        line2: &str,
        options: &TleToOmmOptions,
    ) -> Result<Self> {
        let line1 = normalize_tle_input_line(line1, "line 1")?;
        let line2 = normalize_tle_input_line(line2, "line 2")?;
        validate_tle_checksum(&line1, "line 1")?;
        validate_tle_checksum(&line2, "line 2")?;

        let l1 = line1.as_bytes();
        ensure_tle_line_structure(&line1, '1')?;
        ensure_tle_line_structure(&line2, '2')?;

        let norad_cat_id_l1 = parse_tle_satellite_number(&line1[2..7], "NORAD_CAT_ID")?;
        let norad_cat_id_l2 = parse_tle_satellite_number(&line2[2..7], "NORAD_CAT_ID")?;
        if norad_cat_id_l1 != norad_cat_id_l2 {
            return Err(ValidationError::Conflict {
                fields: vec![
                    Cow::Borrowed("NORAD_CAT_ID (line1)"),
                    Cow::Borrowed("NORAD_CAT_ID (line2)"),
                ],
                line: None,
            }
            .into());
        }

        let classification_type = (l1[7] as char).to_string();
        parse_classification_char(&classification_type)?;

        let launch_designator =
            parse_tle_launch_designator_fields(&line1[9..11], &line1[11..14], &line1[14..17])?;

        let epoch_year_2 = parse_u32_strict(&line1[18..20], "EPOCH_YEAR")?;
        let epoch = parse_tle_epoch_field(epoch_year_2, &line1[20..32])?;

        let mean_motion_dot = parse_tle_dot_term(&line1[33..43], "MEAN_MOTION_DOT")?;
        let mean_motion_ddot = parse_tle_assumed_decimal(&line1[44..52], "MEAN_MOTION_DDOT")?;
        let bstar = parse_tle_assumed_decimal(&line1[53..61], "BSTAR")?;
        let ephemeris_type = parse_u32_trimmed(&line1[62..63], "EPHEMERIS_TYPE")?;
        if ephemeris_type > 9 {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("EPHEMERIS_TYPE"),
                value: ephemeris_type.to_string(),
                expected: Cow::Borrowed("[0, 9]"),
                line: None,
            }
            .into());
        }
        let element_set_no = parse_u32_trimmed(&line1[64..68], "ELEMENT_SET_NO")?;

        let inclination = parse_f64_trimmed(&line2[8..16], "INCLINATION")?;
        let ra_of_asc_node = parse_f64_trimmed(&line2[17..25], "RA_OF_ASC_NODE")?;
        let eccentricity_digits = parse_u32_strict(&line2[26..33], "ECCENTRICITY")?;
        let arg_of_pericenter = parse_f64_trimmed(&line2[34..42], "ARG_OF_PERICENTER")?;
        let mean_anomaly = parse_f64_trimmed(&line2[43..51], "MEAN_ANOMALY")?;
        let mean_motion = parse_f64_trimmed(&line2[52..63], "MEAN_MOTION")?;
        let rev_at_epoch = parse_u32_trimmed(&line2[63..68], "REV_AT_EPOCH")?;

        let derived_object_id = launch_designator
            .as_ref()
            .map(|v| {
                format_launch_designator_object_id(
                    v.launch_year_2,
                    v.launch_number,
                    &v.launch_piece,
                )
            })
            .unwrap_or_else(|| "UNKNOWN".to_string());

        let object_name = options
            .object_name
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let object_id = options.object_id.clone().unwrap_or(derived_object_id);
        let originator = options
            .originator
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let creation_date = options.creation_date.unwrap_or(epoch);

        Ok(Self {
            header: OdmHeader {
                comment: Vec::new(),
                classification: None,
                creation_date,
                originator,
                message_id: options.message_id.clone(),
            },
            body: OmmBody {
                segment: OmmSegment {
                    metadata: OmmMetadata {
                        comment: Vec::new(),
                        object_name,
                        object_id,
                        center_name: "EARTH".to_string(),
                        ref_frame: "TEME".to_string(),
                        ref_frame_epoch: None,
                        time_system: "UTC".to_string(),
                        mean_element_theory: "SGP4".to_string(),
                    },
                    data: OmmData {
                        comment: Vec::new(),
                        mean_elements: MeanElements {
                            comment: Vec::new(),
                            epoch,
                            semi_major_axis: None,
                            mean_motion: Some(MeanMotion::new(mean_motion, None)),
                            eccentricity: NonNegativeDouble::new(
                                eccentricity_digits as f64 / 1.0e7,
                            )?,
                            inclination: Inclination::new(inclination, None)?,
                            ra_of_asc_node: Angle::new(ra_of_asc_node, None)?,
                            arg_of_pericenter: Angle::new(arg_of_pericenter, None)?,
                            mean_anomaly: Angle::new(mean_anomaly, None)?,
                            gm: None,
                        },
                        spacecraft_parameters: None,
                        tle_parameters: Some(TleParameters {
                            comment: Vec::new(),
                            ephemeris_type: Some(ephemeris_type as i32),
                            classification_type: Some(classification_type),
                            norad_cat_id: Some(norad_cat_id_l1),
                            element_set_no: Some(ElementSetNo::new(element_set_no)?),
                            rev_at_epoch: Some(rev_at_epoch),
                            bstar: Some(BStar::new(bstar, None)),
                            bterm: None,
                            mean_motion_dot: MeanMotionDot::new(mean_motion_dot, None),
                            mean_motion_ddot: Some(MeanMotionDDot::new(mean_motion_ddot, None)),
                            agom: None,
                        }),
                        covariance_matrix: None,
                        user_defined_parameters: None,
                    },
                },
            },
            id: Some("CCSDS_OMM_VERS".to_string()),
            version: "2.0".to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LaunchDesignator {
    launch_year: u32,
    launch_number: u32,
    launch_piece: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedTleLaunchDesignator {
    launch_year_2: u32,
    launch_number: u32,
    launch_piece: String,
}

const NS_PER_DAY: i128 = 86_400_000_000_000;

fn require_tle_field<T: Copy>(value: Option<T>, field: &'static str) -> Result<T> {
    value
        .ok_or(ValidationError::MissingRequiredField {
            block: Cow::Borrowed("TLE Parameters"),
            field: Cow::Borrowed(field),
            line: None,
        })
        .map_err(Into::into)
}

fn parse_classification_char(value: &str) -> Result<char> {
    let trimmed = value.trim();
    if trimmed.len() != 1 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("CLASSIFICATION_TYPE"),
            value: value.to_string(),
            expected: Cow::Borrowed("single ASCII letter"),
            line: None,
        }
        .into());
    }
    let c = trimmed.chars().next().expect("len checked");
    if !c.is_ascii_alphabetic() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("CLASSIFICATION_TYPE"),
            value: value.to_string(),
            expected: Cow::Borrowed("single ASCII letter"),
            line: None,
        }
        .into());
    }
    Ok(c.to_ascii_uppercase())
}

fn validate_angle_for_tle(name: &'static str, value: f64, allow_wrap: bool) -> Result<()> {
    let in_range = if allow_wrap {
        (0.0..360.0).contains(&value)
    } else {
        (0.0..=180.0).contains(&value)
    };
    if !in_range {
        let expected = if allow_wrap {
            "[0.0, 360.0)"
        } else {
            "[0.0, 180.0]"
        };
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed(name),
            value: value.to_string(),
            expected: Cow::Borrowed(expected),
            line: None,
        }
        .into());
    }
    Ok(())
}

fn parse_object_id_launch_designator(object_id: &str) -> Result<Option<LaunchDesignator>> {
    let id = object_id.trim();
    if id.is_empty() || id.eq_ignore_ascii_case("UNKNOWN") {
        return Ok(None);
    }
    let (year_str, rest) = id
        .split_once('-')
        .ok_or_else(|| ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNN[PPP] or UNKNOWN"),
            line: None,
        })?;

    if year_str.len() != 4 || !year_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNN[PPP] (4-digit year)"),
            line: None,
        }
        .into());
    }
    if rest.len() < 3 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNN[PPP]"),
            line: None,
        }
        .into());
    }

    let launch_number_str = &rest[..3];
    let launch_piece_str = &rest[3..];
    if !launch_number_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNN[PPP] (3-digit launch number)"),
            line: None,
        }
        .into());
    }
    if launch_piece_str.len() > 3
        || !launch_piece_str.is_empty() && !launch_piece_str.chars().all(|c| c.is_ascii_uppercase())
    {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNN[PPP] (piece is 0..=3 uppercase ASCII letters)"),
            line: None,
        }
        .into());
    }

    let launch_year = year_str.parse::<u32>()?;
    let launch_number = launch_number_str.parse::<u32>()?;
    if launch_number > 999 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("OBJECT_ID launch number"),
            value: launch_number.to_string(),
            expected: Cow::Borrowed("[0, 999]"),
            line: None,
        }
        .into());
    }

    Ok(Some(LaunchDesignator {
        launch_year,
        launch_number,
        launch_piece: launch_piece_str.to_string(),
    }))
}

fn parse_tle_launch_designator_fields(
    launch_year_field: &str,
    launch_number_field: &str,
    launch_piece_field: &str,
) -> Result<Option<ParsedTleLaunchDesignator>> {
    let year_is_blank = launch_year_field.trim().is_empty();
    let number_is_blank = launch_number_field.trim().is_empty();
    let piece_trimmed = launch_piece_field.trim_end();
    let piece_is_blank = piece_trimmed.is_empty();

    if year_is_blank && number_is_blank && piece_is_blank {
        return Ok(None);
    }
    if year_is_blank || number_is_blank {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("LAUNCH_DESIGNATOR"),
            value: format!(
                "{}{}{}",
                launch_year_field, launch_number_field, launch_piece_field
            ),
            expected: Cow::Borrowed(
                "all launch-designator columns populated or all blank for unknown",
            ),
            line: None,
        }
        .into());
    }

    let launch_year_2 = parse_u32_strict(launch_year_field, "LAUNCH_YEAR")?;
    let launch_number = parse_u32_strict(launch_number_field, "LAUNCH_NUMBER")?;

    if !piece_trimmed.is_empty()
        && (!piece_trimmed.chars().all(|c| c.is_ascii_uppercase()) || piece_trimmed.len() > 3)
    {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("LAUNCH_PIECE"),
            value: piece_trimmed.to_string(),
            expected: Cow::Borrowed("0..=3 uppercase ASCII letters"),
            line: None,
        }
        .into());
    }

    Ok(Some(ParsedTleLaunchDesignator {
        launch_year_2,
        launch_number,
        launch_piece: piece_trimmed.to_string(),
    }))
}

fn normalize_tle_line_len(line: String, label: &'static str) -> Result<String> {
    if line.len() != 68 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: line,
            expected: Cow::Borrowed("line content must be exactly 68 chars before checksum"),
            line: None,
        }
        .into());
    }
    Ok(line)
}

fn normalize_tle_input_line(line: &str, label: &'static str) -> Result<String> {
    let trimmed = line.trim_end_matches(['\r', '\n']);
    if !trimmed.is_ascii() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: trimmed.to_string(),
            expected: Cow::Borrowed("ASCII-only TLE line with fixed-width columns"),
            line: None,
        }
        .into());
    }
    if trimmed.len() == 68 {
        // Some real-world feeds omit the checksum character. Accept and normalize by
        // computing/appending checksum so downstream parsing remains deterministic.
        let checksum = tle_checksum(trimmed);
        return Ok(format!("{}{}", trimmed, checksum));
    }
    if trimmed.len() != 69 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: trimmed.to_string(),
            expected: Cow::Borrowed("exactly 68 (no checksum) or 69 (with checksum) characters"),
            line: None,
        }
        .into());
    }
    Ok(trimmed.to_string())
}

fn tle_checksum(line_without_checksum: &str) -> u32 {
    line_without_checksum
        .as_bytes()
        .iter()
        .map(|b| match *b {
            b'0'..=b'9' => (b - b'0') as u32,
            b'-' => 1,
            _ => 0,
        })
        .sum::<u32>()
        % 10
}

fn validate_tle_checksum(line: &str, label: &'static str) -> Result<()> {
    let expected = line
        .as_bytes()
        .last()
        .and_then(|b| match *b {
            b'0'..=b'9' => Some((b - b'0') as u32),
            _ => None,
        })
        .ok_or_else(|| ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: line.to_string(),
            expected: Cow::Borrowed("checksum digit in column 69"),
            line: None,
        })?;

    let actual = tle_checksum(&line[..68]);
    if actual != expected {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: line.to_string(),
            expected: Cow::Owned(format!("valid checksum {}", actual)),
            line: None,
        }
        .into());
    }
    Ok(())
}

fn ensure_tle_line_structure(line: &str, line_number: char) -> Result<()> {
    let b = line.as_bytes();
    if b[0] as char != line_number {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("TLE line number"),
            value: line.to_string(),
            expected: Cow::Owned(format!("{}", line_number)),
            line: None,
        }
        .into());
    }
    if b[1] != b' ' {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("TLE spacing"),
            value: line.to_string(),
            expected: Cow::Borrowed("space in column 2"),
            line: None,
        }
        .into());
    }
    Ok(())
}

fn parse_u32_strict(s: &str, field: &'static str) -> Result<u32> {
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(field),
            value: s.to_string(),
            expected: Cow::Borrowed("ASCII digits"),
            line: None,
        }
        .into());
    }
    Ok(s.parse::<u32>()?)
}

fn alpha5_prefix_to_value(c: char) -> Option<u32> {
    match c {
        'A'..='H' => Some(10 + (c as u32 - 'A' as u32)),
        'J'..='N' => Some(18 + (c as u32 - 'J' as u32)),
        'P'..='Z' => Some(23 + (c as u32 - 'P' as u32)),
        _ => None,
    }
}

fn value_to_alpha5_prefix(value: u32) -> Option<char> {
    match value {
        10..=17 => Some(char::from_u32('A' as u32 + (value - 10)).expect("ASCII range")),
        18..=22 => Some(char::from_u32('J' as u32 + (value - 18)).expect("ASCII range")),
        23..=33 => Some(char::from_u32('P' as u32 + (value - 23)).expect("ASCII range")),
        _ => None,
    }
}

fn parse_tle_satellite_number(field: &str, name: &'static str) -> Result<u32> {
    if field.chars().all(|c| c.is_ascii_digit()) {
        return field.parse::<u32>().map_err(Into::into);
    }

    let bytes = field.as_bytes();
    if bytes.len() == 5
        && (bytes[0] as char).is_ascii_uppercase()
        && field[1..].chars().all(|c| c.is_ascii_digit())
    {
        let prefix = alpha5_prefix_to_value(bytes[0] as char).ok_or_else(|| {
            ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed(
                    "5-digit number, right-aligned number, or Alpha-5 ID (A-Z excluding I/O)",
                ),
                line: None,
            }
        })?;
        let suffix = field[1..].parse::<u32>()?;
        return Ok(prefix * 10_000 + suffix);
    }

    let trimmed = field.trim();
    if !trimmed.is_empty() && trimmed.chars().all(|c| c.is_ascii_digit()) {
        return trimmed.parse::<u32>().map_err(Into::into);
    }

    Err(ValidationError::InvalidValue {
        field: Cow::Borrowed(name),
        value: field.to_string(),
        expected: Cow::Borrowed(
            "5-digit number, right-aligned number, or Alpha-5 ID (A-Z excluding I/O)",
        ),
        line: None,
    }
    .into())
}

fn format_tle_satellite_number(norad_cat_id: u32) -> Result<String> {
    if norad_cat_id <= 99_999 {
        return Ok(format!("{:05}", norad_cat_id));
    }

    if norad_cat_id > 339_999 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("NORAD_CAT_ID"),
            value: norad_cat_id.to_string(),
            expected: Cow::Borrowed("[0, 339999] encodable as TLE 5-char sat number"),
            line: None,
        }
        .into());
    }

    let prefix = norad_cat_id / 10_000;
    let suffix = norad_cat_id % 10_000;
    let prefix_char =
        value_to_alpha5_prefix(prefix).ok_or_else(|| ValidationError::OutOfRange {
            name: Cow::Borrowed("NORAD_CAT_ID"),
            value: norad_cat_id.to_string(),
            expected: Cow::Borrowed("[0, 339999] with Alpha-5 prefix"),
            line: None,
        })?;
    Ok(format!("{}{:04}", prefix_char, suffix))
}

fn parse_u32_trimmed(s: &str, field: &'static str) -> Result<u32> {
    let t = s.trim();
    if t.is_empty() || !t.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(field),
            value: s.to_string(),
            expected: Cow::Borrowed("right-aligned unsigned integer"),
            line: None,
        }
        .into());
    }
    Ok(t.parse::<u32>()?)
}

fn parse_f64_trimmed(s: &str, field: &'static str) -> Result<f64> {
    let t = s.trim();
    if t.is_empty() {
        return Err(ValidationError::MissingRequiredField {
            block: Cow::Borrowed("TLE"),
            field: Cow::Borrowed(field),
            line: None,
        }
        .into());
    }
    t.parse::<f64>()
        .map_err(|_| ValidationError::InvalidValue {
            field: Cow::Borrowed(field),
            value: s.to_string(),
            expected: Cow::Borrowed("floating-point value"),
            line: None,
        })
        .map_err(Into::into)
}

fn format_tle_dot_term(value: f64) -> Result<String> {
    if !value.is_finite() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("MEAN_MOTION_DOT"),
            value: value.to_string(),
            expected: Cow::Borrowed("finite number"),
            line: None,
        }
        .into());
    }
    let sign = if value.is_sign_negative() { '-' } else { ' ' };
    let scaled = (value.abs() * 1.0e8).round();
    if !(0.0..100_000_000.0).contains(&scaled) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("MEAN_MOTION_DOT"),
            value: value.to_string(),
            expected: Cow::Borrowed("encodable as s.dddddddd"),
            line: None,
        }
        .into());
    }
    Ok(format!("{}.{:08}", sign, scaled as u64))
}

fn parse_tle_dot_term(field: &str, name: &'static str) -> Result<f64> {
    let b = field.as_bytes();
    if b.len() != 10 || b[1] != b'.' {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(name),
            value: field.to_string(),
            expected: Cow::Borrowed("s.dddddddd"),
            line: None,
        }
        .into());
    }
    if !field[2..].chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(name),
            value: field.to_string(),
            expected: Cow::Borrowed("s.dddddddd"),
            line: None,
        }
        .into());
    }
    let sign = match b[0] as char {
        '-' => -1.0,
        ' ' | '+' => 1.0,
        _ => {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed("leading sign/space"),
                line: None,
            }
            .into());
        }
    };
    let digits = parse_u32_strict(&field[2..], name)?;
    Ok(sign * (digits as f64) / 1.0e8)
}

fn format_tle_assumed_decimal(value: f64) -> Result<String> {
    if !value.is_finite() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("TLE assumed-decimal field"),
            value: value.to_string(),
            expected: Cow::Borrowed("finite number"),
            line: None,
        }
        .into());
    }

    if value == 0.0 {
        return Ok(" 00000-0".to_string());
    }

    let sign_char = if value.is_sign_negative() { '-' } else { ' ' };
    let abs = value.abs();
    let exponent = abs.log10().floor() as i32 + 1;
    let mantissa_raw = abs / 10f64.powi(exponent);
    let mut mantissa = (mantissa_raw * 1.0e5).round() as i32;
    let mut exp = exponent;

    if mantissa == 100_000 {
        mantissa = 10_000;
        exp += 1;
    }
    if !(10_000..=99_999).contains(&mantissa) || !(-9..=9).contains(&exp) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("TLE assumed-decimal field"),
            value: value.to_string(),
            expected: Cow::Borrowed("encodable as sXXXXXsY"),
            line: None,
        }
        .into());
    }

    let exp_sign = if exp < 0 { '-' } else { '+' };
    let exp_digit = exp.unsigned_abs();
    Ok(format!(
        "{}{:05}{}{}",
        sign_char, mantissa, exp_sign, exp_digit
    ))
}

fn parse_tle_assumed_decimal(field: &str, name: &'static str) -> Result<f64> {
    if field.len() != 8 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(name),
            value: field.to_string(),
            expected: Cow::Borrowed("sXXXXXsY"),
            line: None,
        }
        .into());
    }
    let b = field.as_bytes();
    let sign = match b[0] as char {
        '-' => -1.0,
        ' ' | '+' => 1.0,
        _ => {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed("leading sign/space"),
                line: None,
            }
            .into());
        }
    };
    let mantissa_str = &field[1..6];
    if !mantissa_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(name),
            value: field.to_string(),
            expected: Cow::Borrowed("5 mantissa digits"),
            line: None,
        }
        .into());
    }
    let exp_sign = match b[6] as char {
        '+' | ' ' => 1_i32,
        '-' => -1_i32,
        _ => {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed("exponent sign"),
                line: None,
            }
            .into());
        }
    };
    let exp_digit = match b[7] {
        b'0'..=b'9' => (b[7] - b'0') as i32,
        _ => {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed("exponent digit"),
                line: None,
            }
            .into());
        }
    };
    let mantissa = mantissa_str.parse::<u32>()? as f64;
    let exponent = exp_sign * exp_digit;
    Ok(sign * (mantissa / 1.0e5) * 10f64.powi(exponent))
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_year(year: i32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

fn days_in_month(year: i32, month: u32) -> Option<u32> {
    let d = match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => return None,
    };
    Some(d)
}

fn month_day_to_doy(year: i32, month: u32, day: u32) -> Option<u32> {
    let mut doy = 0_u32;
    for m in 1..month {
        doy += days_in_month(year, m)?;
    }
    if day == 0 || day > days_in_month(year, month)? {
        return None;
    }
    Some(doy + day)
}

fn doy_to_month_day(year: i32, mut doy: u32) -> Option<(u32, u32)> {
    if doy == 0 || doy > days_in_year(year) {
        return None;
    }
    for month in 1..=12 {
        let dim = days_in_month(year, month)?;
        if doy <= dim {
            return Some((month, doy));
        }
        doy -= dim;
    }
    None
}

fn parse_om_epoch_to_utc_ydns(epoch: &str) -> Result<(i32, u32, i128)> {
    let (date_part, time_part_full) =
        epoch
            .split_once('T')
            .ok_or_else(|| ValidationError::InvalidValue {
                field: Cow::Borrowed("EPOCH"),
                value: epoch.to_string(),
                expected: Cow::Borrowed("YYYY-MM-DDThh:mm:ss(.fffffff)[Z|(+|-)hh:mm]"),
                line: None,
            })?;

    let (time_part, tz_offset_minutes) = parse_time_and_tz(time_part_full)?;
    let (mut year, mut doy) = parse_epoch_date_to_year_doy(date_part)?;
    let ns_of_day = parse_time_to_ns_of_day(time_part)?;

    let mut utc_ns_of_day = ns_of_day - (tz_offset_minutes as i128) * 60 * 1_000_000_000;
    while utc_ns_of_day < 0 {
        utc_ns_of_day += NS_PER_DAY;
        if doy == 1 {
            year -= 1;
            doy = days_in_year(year);
        } else {
            doy -= 1;
        }
    }
    while utc_ns_of_day >= NS_PER_DAY {
        utc_ns_of_day -= NS_PER_DAY;
        if doy == days_in_year(year) {
            year += 1;
            doy = 1;
        } else {
            doy += 1;
        }
    }

    Ok((year, doy, utc_ns_of_day))
}

fn parse_time_and_tz(value: &str) -> Result<(&str, i32)> {
    if let Some(stripped) = value.strip_suffix('Z') {
        return Ok((stripped, 0));
    }

    if value.len() >= 6 {
        let sign_index = value.len() - 6;
        let tz = &value[sign_index..];
        let tzb = tz.as_bytes();
        if (tzb[0] == b'+' || tzb[0] == b'-') && tzb[3] == b':' {
            let hh = parse_u32_strict(&tz[1..3], "EPOCH timezone hour")?;
            let mm = parse_u32_strict(&tz[4..6], "EPOCH timezone minute")?;
            if hh > 23 || mm > 59 {
                return Err(ValidationError::OutOfRange {
                    name: Cow::Borrowed("EPOCH timezone"),
                    value: tz.to_string(),
                    expected: Cow::Borrowed("hh in [00,23], mm in [00,59]"),
                    line: None,
                }
                .into());
            }
            let sign = if tzb[0] == b'-' { -1 } else { 1 };
            let offset = sign * ((hh as i32) * 60 + (mm as i32));
            return Ok((&value[..sign_index], offset));
        }
    }

    Ok((value, 0))
}

fn parse_epoch_date_to_year_doy(date: &str) -> Result<(i32, u32)> {
    let bytes = date.as_bytes();
    if bytes.is_empty() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: date.to_string(),
            expected: Cow::Borrowed("non-empty date part"),
            line: None,
        }
        .into());
    }

    let mut idx = 0usize;
    if bytes[0] == b'+' || bytes[0] == b'-' {
        idx = 1;
    }
    let first_dash_rel = date[idx..]
        .find('-')
        .ok_or_else(|| ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: date.to_string(),
            expected: Cow::Borrowed("date separator '-'"),
            line: None,
        })?;
    let first_dash = idx + first_dash_rel;
    let year = date[..first_dash]
        .parse::<i32>()
        .map_err(|_| ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: date.to_string(),
            expected: Cow::Borrowed("valid year"),
            line: None,
        })?;
    let tail = &date[first_dash + 1..];

    if let Some((month_str, day_str)) = tail.split_once('-') {
        let month = parse_u32_strict(month_str, "EPOCH month")?;
        let day = parse_u32_strict(day_str, "EPOCH day")?;
        let doy =
            month_day_to_doy(year, month, day).ok_or_else(|| ValidationError::InvalidValue {
                field: Cow::Borrowed("EPOCH"),
                value: date.to_string(),
                expected: Cow::Borrowed("valid calendar date"),
                line: None,
            })?;
        Ok((year, doy))
    } else {
        let doy = parse_u32_strict(tail, "EPOCH day-of-year")?;
        if doy == 0 || doy > days_in_year(year) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("EPOCH day-of-year"),
                value: doy.to_string(),
                expected: Cow::Owned(format!("[1, {}]", days_in_year(year))),
                line: None,
            }
            .into());
        }
        Ok((year, doy))
    }
}

fn parse_time_to_ns_of_day(time: &str) -> Result<i128> {
    let mut parts = time.split(':');
    let hh = parts.next().ok_or_else(|| ValidationError::InvalidValue {
        field: Cow::Borrowed("EPOCH"),
        value: time.to_string(),
        expected: Cow::Borrowed("hh:mm:ss(.fraction)"),
        line: None,
    })?;
    let mm = parts.next().ok_or_else(|| ValidationError::InvalidValue {
        field: Cow::Borrowed("EPOCH"),
        value: time.to_string(),
        expected: Cow::Borrowed("hh:mm:ss(.fraction)"),
        line: None,
    })?;
    let ss = parts.next().ok_or_else(|| ValidationError::InvalidValue {
        field: Cow::Borrowed("EPOCH"),
        value: time.to_string(),
        expected: Cow::Borrowed("hh:mm:ss(.fraction)"),
        line: None,
    })?;
    if parts.next().is_some() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: time.to_string(),
            expected: Cow::Borrowed("hh:mm:ss(.fraction)"),
            line: None,
        }
        .into());
    }

    let hour = parse_u32_strict(hh, "EPOCH hour")?;
    let minute = parse_u32_strict(mm, "EPOCH minute")?;
    if hour > 23 || minute > 59 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH hour/minute"),
            value: format!("{}:{}", hour, minute),
            expected: Cow::Borrowed("hour [0,23], minute [0,59]"),
            line: None,
        }
        .into());
    }

    let (sec_str, frac_str) = match ss.split_once('.') {
        Some((s, f)) => (s, Some(f)),
        None => (ss, None),
    };
    let mut second = parse_u32_strict(sec_str, "EPOCH second")?;
    if second > 60 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH second"),
            value: second.to_string(),
            expected: Cow::Borrowed("[0, 60]"),
            line: None,
        }
        .into());
    }

    let mut nanos = 0_u32;
    if let Some(frac) = frac_str {
        if !frac.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed("EPOCH"),
                value: time.to_string(),
                expected: Cow::Borrowed("fractional seconds must be digits"),
                line: None,
            }
            .into());
        }
        if frac.len() <= 9 {
            let mut padded = frac.to_string();
            while padded.len() < 9 {
                padded.push('0');
            }
            nanos = padded.parse::<u32>()?;
        } else {
            let head = &frac[..9];
            let mut rounded = head.parse::<u32>()?;
            let next_digit = frac.as_bytes()[9];
            if next_digit >= b'5' {
                rounded += 1;
            }
            if rounded == 1_000_000_000 {
                rounded = 0;
                second += 1;
            }
            nanos = rounded;
        }
    }

    if second == 60 && nanos > 0 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: time.to_string(),
            expected: Cow::Borrowed("leap second must not carry fractional part"),
            line: None,
        }
        .into());
    }
    if second > 60 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH second"),
            value: second.to_string(),
            expected: Cow::Borrowed("[0, 60]"),
            line: None,
        }
        .into());
    }
    if second == 60 {
        second = 59;
        nanos = 999_999_999;
    }

    Ok((((hour * 3600 + minute * 60 + second) as i128) * 1_000_000_000) + nanos as i128)
}

fn format_tle_epoch_components(epoch: &str) -> Result<(u32, String)> {
    let (mut year, mut doy, utc_ns_of_day) = parse_om_epoch_to_utc_ydns(epoch)?;
    if !(0..=9999).contains(&year) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH year"),
            value: year.to_string(),
            expected: Cow::Borrowed("[0, 9999]"),
            line: None,
        }
        .into());
    }

    let mut frac_scaled = ((utc_ns_of_day * 100_000_000 + (NS_PER_DAY / 2)) / NS_PER_DAY) as u32;
    if frac_scaled == 100_000_000 {
        frac_scaled = 0;
        if doy == days_in_year(year) {
            year += 1;
            doy = 1;
        } else {
            doy += 1;
        }
    }

    if doy == 0 || doy > days_in_year(year) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH day-of-year"),
            value: doy.to_string(),
            expected: Cow::Owned(format!("[1, {}]", days_in_year(year))),
            line: None,
        }
        .into());
    }

    Ok((
        (year as u32) % 100,
        format!("{:03}.{:08}", doy, frac_scaled),
    ))
}

fn parse_tle_epoch_year(two_digit_year: u32) -> i32 {
    if two_digit_year >= 57 {
        1900 + two_digit_year as i32
    } else {
        2000 + two_digit_year as i32
    }
}

fn parse_tle_epoch_field(epoch_year_2: u32, epoch_day_field: &str) -> Result<CalendarEpoch> {
    if epoch_day_field.len() != 12 || &epoch_day_field[3..4] != "." {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: epoch_day_field.to_string(),
            expected: Cow::Borrowed("DDD.dddddddd"),
            line: None,
        }
        .into());
    }
    let mut year = parse_tle_epoch_year(epoch_year_2);
    let mut doy = parse_u32_strict(&epoch_day_field[..3], "EPOCH day-of-year")?;
    if doy == 0 || doy > days_in_year(year) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH day-of-year"),
            value: doy.to_string(),
            expected: Cow::Owned(format!("[1, {}]", days_in_year(year))),
            line: None,
        }
        .into());
    }
    let frac = parse_u32_strict(&epoch_day_field[4..], "EPOCH fraction")?;
    if frac >= 100_000_000 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH fraction"),
            value: frac.to_string(),
            expected: Cow::Borrowed("[0, 99999999]"),
            line: None,
        }
        .into());
    }

    let mut ns_of_day = ((frac as i128) * NS_PER_DAY + 50_000_000) / 100_000_000;
    if ns_of_day >= NS_PER_DAY {
        ns_of_day -= NS_PER_DAY;
        if doy == days_in_year(year) {
            year += 1;
            doy = 1;
        } else {
            doy += 1;
        }
    }

    let (month, day) =
        doy_to_month_day(year, doy).ok_or_else(|| ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: epoch_day_field.to_string(),
            expected: Cow::Borrowed("valid day of year"),
            line: None,
        })?;

    let hour = (ns_of_day / 3_600_000_000_000) as u32;
    ns_of_day %= 3_600_000_000_000;
    let minute = (ns_of_day / 60_000_000_000) as u32;
    ns_of_day %= 60_000_000_000;
    let second = (ns_of_day / 1_000_000_000) as u32;
    let nanos = (ns_of_day % 1_000_000_000) as u32;

    let mut epoch = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        year, month, day, hour, minute, second
    );
    if nanos > 0 {
        let mut frac_ns = format!("{:09}", nanos);
        while frac_ns.ends_with('0') {
            frac_ns.pop();
        }
        epoch.push('.');
        epoch.push_str(&frac_ns);
    }

    CalendarEpoch::new(&epoch).map_err(Into::into)
}

fn format_launch_designator_object_id(
    launch_year_2: u32,
    launch_number: u32,
    launch_piece_field: &str,
) -> String {
    let launch_year = parse_tle_epoch_year(launch_year_2);
    let piece = launch_piece_field.trim();
    if piece.is_empty() {
        format!("{:04}-{:03}", launch_year, launch_number)
    } else {
        format!("{:04}-{:03}{}", launch_year, launch_number, piece)
    }
}
