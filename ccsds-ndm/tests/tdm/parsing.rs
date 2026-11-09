use crate::common::{mutated, mutated_once};
use crate::{KVN, XML};
use ccsds_ndm::error::CcsdsNdmError;
use ccsds_ndm::messages::tdm::{Tdm, TdmObservationData};
use ccsds_ndm::types::{
    TdmAngleType, TdmDataQuality, TdmIntegrationRef, TdmMode, TdmPath, TdmRangeMode, TdmRangeUnits,
    TdmReferenceFrame, TdmTimetagRef, YesNo,
};
use ccsds_ndm::Ndm;

#[test]
fn original_e17_repeated_rcs_epoch_is_rejected_and_adaptation_remains_valid() {
    let original = include_str!("../../data/invalid/tdm_e17_original.kvn");
    let error = Tdm::from_kvn(original).unwrap_err();
    crate::common::assert_validation_field(&error, "RCS");
    assert!(error.to_string().contains("strictly increasing epochs"));
    let adapted = Tdm::from_kvn(include_str!("../../data/kvn/tdm_e17.kvn")).unwrap();
    assert_eq!(Tdm::from_kvn(&adapted.to_kvn().unwrap()).unwrap(), adapted);
    assert_eq!(Tdm::from_xml(&adapted.to_xml().unwrap()).unwrap(), adapted);
}
#[test]
fn test_parse_tdm_example_e1_oneway() {
    let tdm = Tdm::from_kvn(KVN).unwrap();
    assert_eq!(tdm.header.creation_date.to_string(), "2005-160T20:15:00Z");
    assert_eq!(tdm.body.segments.len(), 1);
    let seg = &tdm.body.segments[0];
    assert_eq!(seg.metadata.participant_1, "DSS-25");
    assert_eq!(seg.metadata.participant_2.as_deref(), Some("yyyy-nnnA"));
    assert_eq!(seg.data.observations.len(), 31);
    assert_eq!(tdm.version, "2.0");
    assert_eq!(tdm.header.comment.len(), 2);
    assert_eq!(
        tdm.header.comment[0],
        "TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)"
    );
    assert_eq!(
        seg.data.comment,
        ["TRANSMIT_FREQ_2 is spacecraft reference downlink"]
    );
    match &seg.data.observations[0].data {
        TdmObservationData::TransmitFreq2(v) => assert_eq!(*v, 32023442781.733),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn test_parse_tdm_example_e16_optical() {
    let kvn = include_str!("../../data/kvn/tdm_e16.kvn");
    let tdm = Tdm::from_kvn(kvn).unwrap();
    let seg = &tdm.body.segments[0];
    assert_eq!(seg.metadata.angle_type, Some(TdmAngleType::Radec));
    assert_eq!(seg.data.observations.len(), 9);
    assert_eq!(tdm.body.segments.len(), 2);
    assert_eq!(
        seg.metadata.participant_2.as_deref(),
        Some("TRACK NUMBER 001")
    );
    assert_eq!(
        tdm.body.segments[1].metadata.participant_2.as_deref(),
        Some("TRACK NUMBER 003")
    );
    match &seg.data.observations[2].data {
        TdmObservationData::Mag(v) => assert_eq!(*v, 12.1),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn header_mandatory_creation_date() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let error = Tdm::from_kvn(kvn).unwrap_err();
    assert!(
        crate::common::kvn_parse_error(&error)
            .unwrap()
            .message
            .contains("Expected CREATION_DATE"),
        "{error}"
    );
}

#[test]
fn header_mandatory_originator() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let error = Tdm::from_kvn(kvn).unwrap_err();
    assert_eq!(
        crate::common::kvn_parse_error(&error).unwrap().message,
        "META_START is out of order"
    );
}

#[test]
fn metadata_mandatory_participant_1() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    crate::common::assert_validation_field(&Tdm::from_kvn(kvn).unwrap_err(), "PARTICIPANT_1");
}

#[test]
fn body_requires_at_least_one_segment() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
    let error = Tdm::from_kvn(kvn).unwrap_err();
    assert_eq!(
        crate::common::kvn_parse_error(&error).unwrap().message,
        "unterminated or incomplete TDM section sequence"
    );
}

#[test]
fn data_requires_at_least_one_observation() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
DATA_STOP
"#;
    let error = Tdm::from_kvn(kvn).unwrap_err();
    let parse = crate::common::kvn_parse_error(&error).unwrap();
    assert!(
        parse
            .contexts
            .contains(&"TDM data section must contain at least one observation"),
        "{error}"
    );
}

#[test]
fn observation_doppler_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
DOPPLER_INSTANTANEOUS = 2023-01-01T00:00:00 -0.5
DOPPLER_INTEGRATED = 2023-01-01T00:00:01 -0.45
DOPPLER_COUNT = 2023-01-01T00:00:02 12345678.0
DATA_STOP
"#;
    // Integral double spellings and >2^31 accumulators read; a fraction is never truncated.
    for (count, accepted) in [("1.0E3", true), ("4294967296", true), ("1000.5", false)] {
        let input = kvn.replace("12345678.0", count);
        assert_eq!(Tdm::from_kvn(&input).is_ok(), accepted, "{count}");
    }
}

#[test]
fn all_metadata_optional_fields() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
META_START
COMMENT Metadata comment
TRACK_ID = TRACK_001
DATA_TYPES = RANGE,DOPPLER_INTEGRATED
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
PARTICIPANT_3 = QUASAR_1
PARTICIPANT_4 = RELAY_SAT
PARTICIPANT_5 = DSS-25
MODE = SEQUENTIAL
PATH = 1,2,1
EPHEMERIS_NAME_1 = DSS14_EPHEM
EPHEMERIS_NAME_2 = SC_EPHEM
EPHEMERIS_NAME_3 = QUASAR_EPHEM
EPHEMERIS_NAME_4 = RELAY_EPHEM
EPHEMERIS_NAME_5 = DSS25_EPHEM
TRANSMIT_BAND = X
RECEIVE_BAND = Ka
TURNAROUND_NUMERATOR = 880
TURNAROUND_DENOMINATOR = 749
TIMETAG_REF = RECEIVE
INTEGRATION_INTERVAL = 60.0
INTEGRATION_REF = MIDDLE
FREQ_OFFSET = 0.0
RANGE_MODE = COHERENT
RANGE_MODULUS = 32768.0
RANGE_UNITS = km
ANGLE_TYPE = AZEL
REFERENCE_FRAME = EME2000
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 7
DOPPLER_COUNT_BIAS = 240000000.0
DOPPLER_COUNT_SCALE = 1000
DOPPLER_COUNT_ROLLOVER = NO
TRANSMIT_DELAY_1 = 0.000077
TRANSMIT_DELAY_2 = 0.000088
RECEIVE_DELAY_1 = 0.000077
RECEIVE_DELAY_2 = 0.000099
DATA_QUALITY = VALIDATED
CORRECTION_RANGE = 0.001
CORRECTIONS_APPLIED = YES
META_STOP
DATA_START
COMMENT Range measurements
RANGE = 2023-01-01T00:00:00 1000.0
RANGE = 2023-01-01T00:01:00 1001.0
DOPPLER_INTEGRATED = 2023-01-01T00:02:00 -0.5
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    let seg = &tdm.body.segments[0];

    assert_eq!(seg.metadata.track_id.as_deref(), Some("TRACK_001"));
    assert_eq!(
        seg.metadata.data_types.as_deref(),
        Some("RANGE,DOPPLER_INTEGRATED")
    );
    assert!(seg.metadata.start_time.is_some());
    assert!(seg.metadata.stop_time.is_some());
    assert_eq!(seg.metadata.mode, Some(TdmMode::Sequential));
    assert_eq!(seg.metadata.path, Some(TdmPath("1,2,1".to_string())));
    assert_eq!(seg.metadata.transmit_band.as_deref(), Some("X"));
    assert_eq!(seg.metadata.receive_band.as_deref(), Some("Ka"));
    assert_eq!(seg.metadata.turnaround_numerator, Some(880));
    assert_eq!(seg.metadata.turnaround_denominator, Some(749));
    assert_eq!(seg.metadata.timetag_ref, Some(TdmTimetagRef::Receive));
    assert_eq!(seg.metadata.integration_interval, Some(60.0));
    assert_eq!(
        seg.metadata.integration_ref,
        Some(TdmIntegrationRef::Middle)
    );
    assert_eq!(seg.metadata.range_mode, Some(TdmRangeMode::Coherent));
    assert_eq!(seg.metadata.range_modulus, Some(32768.0));
    assert_eq!(seg.metadata.range_units, Some(TdmRangeUnits::Km));
    assert_eq!(seg.metadata.angle_type, Some(TdmAngleType::Azel));
    assert_eq!(
        seg.metadata.reference_frame,
        Some(TdmReferenceFrame::Eme2000)
    );
    assert_eq!(seg.metadata.interpolation.as_deref(), Some("LAGRANGE"));
    assert_eq!(seg.metadata.interpolation_degree, Some(7));
    assert_eq!(seg.metadata.doppler_count_bias, Some(240000000.0));
    assert_eq!(seg.metadata.doppler_count_scale, Some(1000));
    assert_eq!(seg.metadata.doppler_count_rollover, Some(YesNo::No));
    assert_eq!(seg.metadata.data_quality, Some(TdmDataQuality::Validated));
    assert_eq!(seg.metadata.correction_range, Some(0.001));
    assert_eq!(seg.metadata.corrections_applied, Some(YesNo::Yes));
    assert_eq!(tdm.header.message_id.as_deref(), Some("MSG-001"));
    assert_eq!(seg.metadata.comment, ["Metadata comment"]);
    assert_eq!(seg.metadata.participant_1, "DSS-14");
    assert_eq!(seg.metadata.participant_2.as_deref(), Some("SPACECRAFT_A"));
    assert_eq!(seg.metadata.participant_3.as_deref(), Some("QUASAR_1"));
    assert_eq!(seg.metadata.participant_4.as_deref(), Some("RELAY_SAT"));
    assert_eq!(seg.metadata.participant_5.as_deref(), Some("DSS-25"));
    assert!(seg.metadata.path_1.is_none());
    assert!(seg.metadata.path_2.is_none());
    assert_eq!(seg.metadata.transmit_delay_1, Some(0.000077));
    assert_eq!(seg.metadata.transmit_delay_2, Some(0.000088));
    assert_eq!(seg.metadata.receive_delay_1, Some(0.000077));
    assert_eq!(seg.metadata.receive_delay_2, Some(0.000099));
    assert_eq!(Tdm::from_kvn(&tdm.to_kvn().unwrap()).unwrap(), tdm);
    let distinct = mutated(
        kvn,
        "RECEIVE_DELAY_1 = 0.000077",
        "RECEIVE_DELAY_1 = 0.000088",
    );
    let distinct = Tdm::from_kvn(&distinct).unwrap();
    assert_eq!(
        distinct.body.segments[0].metadata.transmit_delay_1,
        Some(0.000077)
    );
    assert_eq!(
        distinct.body.segments[0].metadata.receive_delay_1,
        Some(0.000088)
    );
}

#[test]
fn test_tdm_version_not_first_error() {
    let kvn = r#"
CREATION_DATE = 2023-01-01T00:00:00
CCSDS_TDM_VERS = 2.0
"#;
    let err = Tdm::from_kvn(kvn).unwrap_err();
    let err = crate::common::kvn_parse_error(&err).expect("KVN syntax diagnostic");
    assert!(err.message.contains("assignment outside a TDM section"));
}

#[test]
fn later_participant_delays_corrections_and_observations_are_parsed() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = P1
PARTICIPANT_2 = P2
PARTICIPANT_3 = P3
PARTICIPANT_4 = P4
PARTICIPANT_5 = P5
TRANSMIT_DELAY_3 = 0.3
TRANSMIT_DELAY_4 = 0.4
TRANSMIT_DELAY_5 = 0.5
RECEIVE_DELAY_4 = 1.4
RECEIVE_DELAY_5 = 1.5
CORRECTION_DOPPLER = 0.3
CORRECTION_MAG = 0.4
CORRECTION_RCS = 0.5
CORRECTION_TRANSMIT = 0.6
CORRECTION_ABERRATION_DIURNAL = 0.7
CORRECTIONS_APPLIED = YES
META_STOP
DATA_START
RECEIVE_FREQ_2 = 2023-01-01T00:00:00 123.0
RECEIVE_FREQ_4 = 2023-01-01T00:00:01 124.0
RECEIVE_FREQ_5 = 2023-01-01T00:00:02 125.0
RECEIVE_PHASE_CT_2 = 2023-01-01T00:00:03 222.0
RECEIVE_PHASE_CT_3 = 2023-01-01T00:00:04 223.0
RECEIVE_PHASE_CT_4 = 2023-01-01T00:00:05 224.0
RECEIVE_PHASE_CT_5 = 2023-01-01T00:00:06 225.0
TRANSMIT_FREQ_3 = 2023-01-01T00:00:07 323.0
TRANSMIT_FREQ_4 = 2023-01-01T00:00:08 324.0
TRANSMIT_FREQ_5 = 2023-01-01T00:00:09 325.0
TRANSMIT_FREQ_RATE_2 = 2023-01-01T00:00:10 422.0
TRANSMIT_FREQ_RATE_3 = 2023-01-01T00:00:11 423.0
TRANSMIT_FREQ_RATE_4 = 2023-01-01T00:00:12 424.0
TRANSMIT_FREQ_RATE_5 = 2023-01-01T00:00:13 425.0
TRANSMIT_PHASE_CT_2 = 2023-01-01T00:00:14 522.0
TRANSMIT_PHASE_CT_3 = 2023-01-01T00:00:15 523.0
TRANSMIT_PHASE_CT_4 = 2023-01-01T00:00:16 524.0
TRANSMIT_PHASE_CT_5 = 2023-01-01T00:00:17 525.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).expect("parse exhaustive tdm kvn");
    let meta = &tdm.body.segments[0].metadata;
    assert_eq!(meta.transmit_delay_5, Some(0.5));
    assert_eq!(meta.correction_aberration_diurnal, Some(0.7));
    assert_eq!(tdm.body.segments[0].data.observations.len(), 18);
    assert_eq!(Tdm::from_kvn(&tdm.to_kvn().unwrap()).unwrap(), tdm);
}

#[test]
fn test_observation_value_error_reports_record_line() {
    let good = include_str!("../../data/kvn/tdm_e1.kvn");
    let record = "RECEIVE_FREQ_1 = 2005-159T17:41:00 32021034790.7265";
    let bad = good.replacen(record, "RHUMIDITY = 2005-159T17:41:00 150.0", 1);
    let expected_line = good[..good.find(record).unwrap()]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1;

    let CcsdsNdmError::Validation(error) =
        crate::common::parse_error_cause(Tdm::from_kvn(&bad).unwrap_err())
    else {
        panic!("expected a validation error");
    };
    // The range error names the keyword, not the shared Rust `Percentage` type.
    assert!(error.to_string().contains("RHUMIDITY"), "{error}");
    let ccsds_ndm::error::ValidationError::OutOfRange { line, .. } = *error else {
        panic!("expected an out-of-range error");
    };
    assert_eq!(line, Some(expected_line));
}

#[test]
fn tdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let time_system = "TIME_SYSTEM = UTC";
    let participant = "PARTICIPANT_1 = DSS-25";
    let observation = "TRANSMIT_FREQ_2 = 2005-159T17:41:00 32023442781.733";
    for (label, source) in [
        (
            "duplicate header keyword",
            mutated(
                KVN,
                "ORIGINATOR = NASA",
                "ORIGINATOR = NASA\nORIGINATOR = NASA",
            ),
        ),
        (
            "duplicate metadata keyword",
            mutated(KVN, time_system, &format!("{time_system}\n{time_system}")),
        ),
        (
            "unknown metadata keyword",
            mutated(KVN, time_system, &format!("{time_system}\nUNKNOWN = value")),
        ),
        (
            "comment after an observation",
            mutated(
                KVN,
                observation,
                &format!("{observation}\nCOMMENT misplaced"),
            ),
        ),
        ("unknown block", mutated(KVN, "META_START", "UNKNOWN_START")),
        (
            "mismatched block end",
            mutated(KVN, "META_STOP", "DATA_STOP"),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            mutated(KVN, participant, &format!("{participant} €")),
        ),
        (
            "unknown header keyword",
            mutated(KVN, "META_START", "UNKNOWN_KEY = VALUE\nMETA_START"),
        ),
        (
            "unknown observation keyword",
            mutated(KVN, observation, "BAD_KEY = 2005-159T17:41:00 1"),
        ),
    ] {
        let error = Tdm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
        let expected = match label {
            "unknown metadata keyword" => "unknown TDM metadata keyword",
            "unknown observation keyword" => "unknown TDM observation keyword",
            _ => continue,
        };
        assert_eq!(
            crate::common::kvn_parse_error(&error).unwrap().message,
            expected
        );
    }
}

#[test]
fn tdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let epoch = "<EPOCH>2007-069T15:22:22.000</EPOCH>";
    let observable = "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>";
    for (label, source) in [
        (
            "unknown observation child",
            mutated(XML, "<observation>", "<observation><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown container attribute",
            mutated(XML, "<metadata>", "<metadata unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            mutated(
                XML,
                observable,
                "<TRANSMIT_FREQ_1 unexpected=\"value\">7167941264.0</TRANSMIT_FREQ_1>",
            ),
        ),
        (
            "reordered observation members",
            mutated_once(
                XML,
                &format!("{epoch}\n{observable}"),
                &format!("{observable}\n{epoch}"),
            ),
        ),
    ] {
        let error = Tdm::from_xml(&source).unwrap_err();
        assert!(
            matches!(
                error.as_format_error(),
                Some(ccsds_ndm::error::FormatError::InvalidFormat(_))
            ),
            "{label}: {error}"
        );
    }
}

#[test]
fn xml_duplicate_epochs_and_unknown_root_attributes_name_the_problem() {
    let epoch = "<EPOCH>2007-069T15:22:22.000</EPOCH>";
    for (input, expected) in [
        (
            mutated_once(XML, epoch, &format!("{epoch}{epoch}")),
            "invalid TDM XML sequence: duplicate or out-of-order child 'EPOCH' in 'observation'",
        ),
        (
            mutated_once(XML, "<tdm ", "<tdm extra=\"val\" "),
            "unknown TDM root attribute 'extra'",
        ),
    ] {
        crate::common::assert_invalid_format(&Tdm::from_xml(&input).unwrap_err(), expected);
    }
}

#[test]
fn tdm_xml_accepts_the_schema_defined_optional_observation_units() {
    let angle = mutated_once(
        XML,
        "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
        "<ANGLE_1 units=\"deg\">1.0</ANGLE_1>",
    );
    Tdm::from_xml(&angle).expect("ANGLE_1 units=deg is allowed by the TDM schema");
    for units in ["rad", "DEG", "km"] {
        let invalid = mutated(&angle, "units=\"deg\"", &format!("units=\"{units}\""));
        assert!(Tdm::from_xml(&invalid).is_err(), "ignored {units}");
    }

    let humidity = mutated_once(
        XML,
        "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
        "<RHUMIDITY units=\"%\">50.0</RHUMIDITY>",
    );
    let mut message =
        Tdm::from_xml(&humidity).expect("RHUMIDITY units=% is allowed by the TDM schema");
    crate::common::assert_validation_field(&message.to_kvn().unwrap_err(), "PARTICIPANT_1");
    message.body.segments[0].metadata.participant_1 = "DSS-25".into();
    let normalized = Tdm::from_kvn(&message.to_kvn().unwrap()).unwrap();
    match &normalized.body.segments[0].data.observations[0].data {
        ccsds_ndm::messages::tdm::TdmObservationData::Rhumidity(value) => {
            assert_eq!(value.value, 50.0);
            assert!(value.units.is_none());
        }
        other => panic!("expected normalized humidity, got {other:?}"),
    }
    let xml = normalized.to_xml().unwrap();
    assert!(!xml.contains(r#"RHUMIDITY units="""#));
    assert!(xml.contains("<RHUMIDITY>50</RHUMIDITY>"));
    assert_eq!(Tdm::from_xml(&xml).unwrap(), normalized);
}

#[test]
fn tdm_xml_accepts_every_schema_range_units_spelling() {
    for (units, canonical, spellings) in [
        (TdmRangeUnits::Km, "km", ["km", "KM"]),
        (TdmRangeUnits::Seconds, "s", ["s", "S"]),
        (TdmRangeUnits::Ru, "RU", ["RU", "ru"]),
    ] {
        let mut message = Tdm::from_kvn(include_str!("../../data/kvn/tdm_e9.kvn")).unwrap();
        message.body.segments[0].metadata.range_units = Some(units);
        let xml = message.to_xml().unwrap();
        for spelling in spellings {
            let candidate = mutated(
                &xml,
                &format!("<RANGE_UNITS>{canonical}</RANGE_UNITS>"),
                &format!("<RANGE_UNITS>{spelling}</RANGE_UNITS>"),
            );
            Tdm::from_xml(&candidate).unwrap_or_else(|error| panic!("{spelling}: {error}"));
        }
    }
}

/// Every conditional metadata rule, as a single mutation of a shipped fixture.
#[test]
fn tdm_conditional_metadata_rules_name_the_offending_field() {
    const FIXTURE: &str = include_str!("../../data/kvn/tdm_e2.kvn");
    Tdm::from_kvn(FIXTURE).expect("baseline fixture must satisfy every rule");

    for (mutated, expected) in [
            // PATH and PATH_n are mutually exclusive, and PATH_1 implies PATH_2.
            (
                mutated(FIXTURE, "PATH = 2,1", "PATH = 2,1\nPATH_1 = 2,1"),
                "duplicate or mutually exclusive TDM metadata keyword",
            ),
            (
                mutated(FIXTURE, "MODE = SEQUENTIAL\nPATH = 2,1", "MODE = SINGLE_DIFF\nPATH_1 = 2,1"),
                "must have both PATH_1 and PATH_2 if one is present",
            ),
            // Unconditionally mandatory metadata.
            (
                mutated(FIXTURE, "TIME_SYSTEM = UTC\n", ""),
                "Missing required field: TIME_SYSTEM",
            ),
            // Each MODE demands its own path keywords.
            (
                mutated(FIXTURE, "PATH = 2,1\n", ""),
                "Missing required field: PATH (required when MODE=SEQUENTIAL)",
            ),
            (
                mutated(FIXTURE, "MODE = SEQUENTIAL\nPATH = 2,1\n", "MODE = SINGLE_DIFF\n"),
                "Missing required field: PATH_1 and PATH_2 (required when MODE=SINGLE_DIFF)",
            ),
            // Keywords that require a companion.
            (
                mutated(FIXTURE, "DATA_QUALITY = RAW", "INTERPOLATION = LAGRANGE\nDATA_QUALITY = RAW"),
                "Missing required field: INTERPOLATION_DEGREE (required when INTERPOLATION is used)",
            ),
            (
                mutated(FIXTURE, "DATA_QUALITY = RAW", "DATA_QUALITY = RAW\nCORRECTION_RANGE = 0.5"),
                "Missing required field: CORRECTIONS_APPLIED (required when CORRECTION_* keywords are used)",
            ),
            (
                mutated(FIXTURE, "DATA_QUALITY = RAW", "ANGLE_TYPE = RADEC\nDATA_QUALITY = RAW"),
                "Missing required field: REFERENCE_FRAME (required when ANGLE_TYPE=RADEC)",
            ),
            // The RADEC rule applies in any letter case, even when the value stays `Other`.
            (
                mutated(FIXTURE, "DATA_QUALITY = RAW", "ANGLE_TYPE = Radec\nDATA_QUALITY = RAW"),
                "Missing required field: REFERENCE_FRAME (required when ANGLE_TYPE=RADEC)",
            ),
            // Path indices must refer to populated participants.
            (
                mutated(FIXTURE, "PATH = 2,1", "PATH = 1,6"),
                "expected participant indices in range 1..5",
            ),
        ] {
            let error = Tdm::from_kvn(&mutated).expect_err("mutation accepted");
            assert!(
                error.to_string().contains(expected),
                "diagnostic did not name {expected}: {error}"
            );
        }
}

#[test]
fn all_normative_line_endings_preserve_the_message() {
    let expected = Tdm::from_kvn(crate::KVN).unwrap();
    for ending in ["\n", "\r", "\r\n", "\n\r"] {
        let input = crate::KVN.replace('\n', ending);
        assert_eq!(Tdm::from_kvn(&input).unwrap(), expected, "{ending:?}");
    }
}

#[test]
fn observation_numbers_use_shared_reading_rules_without_underflow() {
    for value in [".5", "5.", "1e3", "1.2345678901234567", "5e-324"] {
        assert!(
            TdmObservationData::from_key_val("RANGE", value).is_ok(),
            "{value}"
        );
    }
    for value in ["1e-999", "1e999", "1e", "NaN", "INF"] {
        assert!(
            TdmObservationData::from_key_val("RANGE", value).is_err(),
            "{value}"
        );
    }
}

#[test]
fn xml_accepts_schema_listed_lowercase_categories() {
    for (field, upper, lower) in [
        ("MODE", "SEQUENTIAL", "sequential"),
        ("TIMETAG_REF", "TRANSMIT", "transmit"),
        ("INTEGRATION_REF", "MIDDLE", "middle"),
        ("RANGE_MODE", "COHERENT", "coherent"),
        ("ANGLE_TYPE", "AZEL", "azel"),
        ("REFERENCE_FRAME", "EME2000", "eme2000"),
        ("DATA_QUALITY", "RAW", "raw"),
    ] {
        let base = if field == "MODE" {
            mutated(XML, "<MODE>SEQUENTIAL</MODE>", "")
        } else {
            XML.to_owned()
        };
        let insertion = if field == "MODE" {
            "<PATH>"
        } else {
            "</metadata>"
        };
        let make = |value: &str| {
            mutated(
                &base,
                insertion,
                &format!("<{field}>{value}</{field}>{insertion}"),
            )
        };
        let xml = make(lower);
        crate::common::validate_xml("lowercase TDM", &xml);
        assert_eq!(
            Tdm::from_xml(&xml).unwrap(),
            Tdm::from_xml(&make(upper)).unwrap(),
            "{field}"
        );
    }
    for (field, value) in [
        ("MODE", "Sequential"),
        ("ANGLE_TYPE", "AzEl"),
        ("REFERENCE_FRAME", "Eme2000"),
    ] {
        let base = mutated(XML, "<MODE>SEQUENTIAL</MODE>", "");
        let insertion = if field == "MODE" {
            "<PATH>"
        } else {
            "</metadata>"
        };
        let xml = mutated(
            &base,
            insertion,
            &format!("<{field}>{value}</{field}>{insertion}"),
        );
        assert!(Tdm::from_xml(&xml).is_err(), "{field}={value}");
    }
}

#[test]
fn present_empty_xml_elements_use_schema_defaults_and_omissions_stay_absent() {
    let source = XML.replace("‘DSS-25’", "DSS-25");
    let baseline = Tdm::from_xml(&source).unwrap();
    assert!(baseline.body.segments[0].metadata.freq_offset.is_none());
    assert!(baseline.body.segments[0].metadata.data_quality.is_none());
    let fields = [
        "FREQ_OFFSET",
        "RANGE_MODULUS",
        "TRANSMIT_DELAY_1",
        "TRANSMIT_DELAY_2",
        "TRANSMIT_DELAY_3",
        "TRANSMIT_DELAY_4",
        "TRANSMIT_DELAY_5",
        "RECEIVE_DELAY_1",
        "RECEIVE_DELAY_2",
        "RECEIVE_DELAY_3",
        "RECEIVE_DELAY_4",
        "RECEIVE_DELAY_5",
        "DATA_QUALITY",
    ];
    for field in fields {
        let explicit = if field == "DATA_QUALITY" {
            "RAW"
        } else {
            "0.0"
        };
        let expected = Tdm::from_xml(&mutated(
            &source,
            "</metadata>",
            &format!("<{field}>{explicit}</{field}></metadata>"),
        ))
        .unwrap();
        let metadata = &expected.body.segments[0].metadata;
        let json = serde_json::to_string(metadata).unwrap();
        assert_eq!(
            serde_json::from_str::<ccsds_ndm::messages::tdm::TdmMetadata>(&json).unwrap(),
            *metadata
        );
        for empty in [format!("<{field}/>"), format!("<{field}></{field}>")] {
            let xml = mutated(&source, "</metadata>", &format!("{empty}</metadata>"));
            crate::common::validate_xml("TDM default", &xml);
            let parsed = Tdm::from_xml(&xml).unwrap();
            assert_eq!(parsed, expected, "{field}");
            assert_eq!(Tdm::from_kvn(&parsed.to_kvn().unwrap()).unwrap(), expected);
        }
        if field != "DATA_QUALITY" {
            for whitespace in [" ", "\t\r\n", "&#32;", "<![CDATA[ ]]>"] {
                let xml = mutated(
                    &source,
                    "</metadata>",
                    &format!("<{field}>{whitespace}</{field}></metadata>"),
                );
                let parsed = Tdm::from_xml(&xml).unwrap();
                assert_eq!(parsed, expected, "{field}: {whitespace:?}");
                assert_eq!(parsed.to_xml().unwrap(), expected.to_xml().unwrap());
            }
        }
    }
    for (field, data) in [
        ("CLOCK_BIAS", TdmObservationData::ClockBias(0.0)),
        ("CLOCK_DRIFT", TdmObservationData::ClockDrift(0.0)),
    ] {
        let mut expected = baseline.clone();
        expected.body.segments[0].metadata.data_types = None;
        expected.body.segments[0].data.observations[0].data = data;
        let xml = expected.to_xml().unwrap();
        let xml = mutated(
            &xml,
            &format!("<{field}>0</{field}>"),
            &format!("<{field}/>"),
        );
        crate::common::validate_xml("TDM clock default", &xml);
        assert_eq!(Tdm::from_xml(&xml).unwrap(), expected);
        for whitespace in [" ", "\t\r\n", "&#32;", "<![CDATA[ ]]>"] {
            let input = mutated(
                &xml,
                &format!("<{field}/>"),
                &format!("<{field}>{whitespace}</{field}>"),
            );
            assert!(Tdm::from_xml(&input).is_err(), "{field}: {whitespace:?}");
        }
    }
    for field in ["INTEGRATION_INTERVAL", "ANGLE_TYPE"] {
        let xml = mutated(&source, "</metadata>", &format!("<{field}/></metadata>"));
        assert!(Tdm::from_xml(&xml).is_err(), "non-defaulted {field}");
    }
}

#[test]
fn kvn_paths_with_blanks_after_commas_read_as_canonical_paths() {
    // Orekit writes `PATH = 2, 1`; TDM 3.3 forbids the blank, but it carries no information.
    const FIXTURE: &str = include_str!("../../data/kvn/tdm_e2.kvn");
    let tdm = Tdm::from_kvn(&mutated(FIXTURE, "PATH = 2,1", "PATH = 2 , 1")).unwrap();
    assert_eq!(tdm, Tdm::from_kvn(FIXTURE).unwrap());
    assert!(tdm
        .to_kvn()
        .unwrap()
        .contains("PATH                 = 2,1\n"));
    let single_diff = mutated(
        FIXTURE,
        "MODE = SEQUENTIAL\nPATH = 2,1",
        "MODE = SINGLE_DIFF\nPATH_1 = 2, 1\nPATH_2 = 1, 2",
    );
    let metadata = &Tdm::from_kvn(&single_diff).unwrap().body.segments[0].metadata;
    assert_eq!(metadata.path_1, Some(TdmPath("2,1".into())));
    assert_eq!(metadata.path_2, Some(TdmPath("1,2".into())));
}

#[test]
fn phase_count_exponents_are_xml_double_spellings_but_not_kvn_phase_counts() {
    const FIXTURE: &str = include_str!("../../data/kvn/tdm_e18.kvn");
    let line = "TRANSMIT_PHASE_CT_1=2005-184T11:12:23 7175173383.615373";
    // TDM 4.3.11: KVN phase counts contain no alphabetic characters.
    let error = Tdm::from_kvn(&mutated(
        FIXTURE,
        line,
        "TRANSMIT_PHASE_CT_1=2005-184T11:12:23 7.175173383615373E9",
    ))
    .unwrap_err();
    assert!(error.to_string().contains("without an exponent"), "{error}");
    // The XSD types XML phase counts as xsd:double, so XML keeps the exponent text (trimmed
    // like every other XML number) and KVN output refuses it rather than rewriting digits.
    let xml = Tdm::from_kvn(FIXTURE).unwrap().to_xml().unwrap().replacen(
        "<TRANSMIT_PHASE_CT_1>7175173383.615373<",
        "<TRANSMIT_PHASE_CT_1> 7.175173383615373E9 <",
        1,
    );
    let tdm = Tdm::from_xml(&xml).unwrap();
    assert_eq!(
        tdm.body.segments[0].data.observations[0].data,
        TdmObservationData::TransmitPhaseCt1("7.175173383615373E9".into())
    );
    let error = tdm.to_kvn().unwrap_err();
    crate::common::assert_validation_field(&error, "TRANSMIT_PHASE_CT_1");
    assert!(error.to_string().contains("without an exponent"), "{error}");
}

#[test]
fn legacy_kvn_is_input_only_and_rejects_version_two_content() {
    let legacy = include_str!("../../data/kvn/tdm_e1.kvn").replacen("= 2.0", "= 1.0", 1);
    let message = Tdm::from_kvn(&legacy).unwrap();
    assert_eq!(message.version, "1.0");
    assert!(ccsds_ndm::from_str(&legacy).is_ok());
    assert_eq!(
        message.to_kvn().unwrap_err().code(),
        Some("generation.unsupported_output_version")
    );
    assert!(message.to_xml().is_err());
    let later = legacy.replace("TIME_SYSTEM = UTC", "TRACK_ID = LATER\nTIME_SYSTEM = UTC");
    assert!(Tdm::from_kvn(&later)
        .unwrap_err()
        .to_string()
        .contains("TRACK_ID"));
    // TDM 1.0 XML is input-only too (conformance::tdm1 covers the 1.0 schema rules).
    let xml = include_str!("../../data/legacy/tdm_1_0_b9.xml");
    assert_eq!(
        Tdm::from_xml(&xml.replace("version=\"1.0\">", "version=\"1.1\">"))
            .unwrap_err()
            .code(),
        Some("parse.unsupported_input_version")
    );
}

#[test]
fn categorical_values_ignore_case_and_blank_aliases_in_kvn() {
    let source = include_str!("../../data/kvn/tdm_e1.kvn")
        .replace("SEQUENTIAL", "Sequential")
        .replace("DEGRADED", "Degraded");
    let message = Tdm::from_kvn(&source).unwrap();
    assert!(message.to_xml().unwrap().contains("SEQUENTIAL"));
    assert_eq!(
        "Single  Diff".parse::<TdmMode>().unwrap(),
        TdmMode::SingleDiff
    );
    assert_eq!(
        "One Way".parse::<TdmRangeMode>().unwrap(),
        TdmRangeMode::OneWay
    );
    assert_eq!("Azel".parse::<TdmAngleType>().unwrap(), TdmAngleType::Azel);
    assert_eq!(
        "Eme2000".parse::<TdmReferenceFrame>().unwrap(),
        TdmReferenceFrame::Eme2000
    );
    assert_eq!("Yes".parse::<YesNo>().unwrap(), YesNo::Yes);
}

#[test]
fn legacy_tables_exclude_every_later_edition_metadata_keyword() {
    let legacy = include_str!("../../data/kvn/tdm_e1.kvn").replacen("= 2.0", "= 1.0", 1);
    for (keyword, value) in [
        ("EPHEMERIS_NAME_1", "FILE"),
        ("INTERPOLATION", "LINEAR"),
        ("INTERPOLATION_DEGREE", "1"),
        ("CORRECTION_MAG", "1"),
        ("CORRECTION_RCS", "1"),
        ("CORRECTION_ABERRATION_YEARLY", "1"),
        ("CORRECTION_ABERRATION_DIURNAL", "1"),
    ] {
        let current = Tdm::from_kvn(include_str!("../../data/kvn/tdm_e1.kvn")).unwrap();
        let mut message = current;
        match keyword {
            "EPHEMERIS_NAME_1" => {
                message.body.segments[0].metadata.ephemeris_name_1 = Some(value.into())
            }
            "INTERPOLATION" => {
                message.body.segments[0].metadata.interpolation = Some(value.into());
                message.body.segments[0].metadata.interpolation_degree = Some(1);
            }
            "INTERPOLATION_DEGREE" => {
                message.body.segments[0].metadata.interpolation_degree = Some(1)
            }
            "CORRECTION_MAG" => message.body.segments[0].metadata.correction_mag = Some(1.0),
            "CORRECTION_RCS" => message.body.segments[0].metadata.correction_rcs = Some(1.0),
            "CORRECTION_ABERRATION_YEARLY" => {
                message.body.segments[0]
                    .metadata
                    .correction_aberration_yearly = Some(1.0)
            }
            _ => {
                message.body.segments[0]
                    .metadata
                    .correction_aberration_diurnal = Some(1.0)
            }
        }
        message.body.segments[0].metadata.corrections_applied = Some(ccsds_ndm::types::YesNo::Yes);
        let source = message.to_kvn().unwrap().replacen("= 2.0", "= 1.0", 1);
        assert!(
            Tdm::from_kvn(&source)
                .unwrap_err()
                .to_string()
                .contains(keyword),
            "{keyword}"
        );
    }
    let source = legacy.replacen("META_START", "MESSAGE_ID = LATER\nMETA_START", 1);
    assert!(Tdm::from_kvn(&source)
        .unwrap_err()
        .to_string()
        .contains("MESSAGE_ID"));
}

#[test]
fn itrf97_aliases_canonicalize_in_both_notations() {
    for alias in ["ITRF97", "ITRF1997"] {
        let input = KVN.replacen(
            "META_STOP",
            &format!("REFERENCE_FRAME = {alias}\nMETA_STOP"),
            1,
        );
        let message = Tdm::from_kvn(&input).unwrap();
        assert_eq!(
            message.body.segments[0].metadata.reference_frame,
            Some(TdmReferenceFrame::Itrf97)
        );
        let xml = message.to_xml().unwrap();
        assert!(xml.contains("<REFERENCE_FRAME>ITRF-97</REFERENCE_FRAME>"));
        assert_eq!(
            Tdm::from_xml(&xml.replace("ITRF-97", alias)).unwrap(),
            message
        );
    }
}

#[test]
fn xml_data_quality_uses_schema_spellings_while_kvn_remains_case_insensitive() {
    let source = Tdm::from_xml(XML).unwrap().to_xml().unwrap();
    for (upper, expected) in [
        ("RAW", TdmDataQuality::Raw),
        ("VALIDATED", TdmDataQuality::Validated),
        ("DEGRADED", TdmDataQuality::Degraded),
    ] {
        for value in [upper.to_owned(), upper.to_ascii_lowercase()] {
            let xml = mutated(
                &source,
                "</metadata>",
                &format!("<DATA_QUALITY>{value}</DATA_QUALITY></metadata>"),
            );
            crate::common::validate_xml("TDM data quality", &xml);
            let parsed = Tdm::from_xml(&xml).unwrap();
            assert_eq!(
                parsed.body.segments[0].metadata.data_quality,
                Some(expected.clone())
            );
        }
        let mixed = format!("{}{}", &upper[..1], upper[1..].to_ascii_lowercase());
        let xml = mutated(
            &source,
            "</metadata>",
            &format!("<DATA_QUALITY>{mixed}</DATA_QUALITY></metadata>"),
        );
        assert!(Tdm::from_xml(&xml).is_err());
        let kvn = mutated(
            KVN,
            "DATA_QUALITY = DEGRADED",
            &format!("DATA_QUALITY = {mixed}"),
        );
        assert_eq!(
            Tdm::from_kvn(&kvn).unwrap().body.segments[0]
                .metadata
                .data_quality,
            Some(expected)
        );
    }
    for value in ["rAw", " RAW ", "", " "] {
        let xml = mutated(
            &source,
            "</metadata>",
            &format!("<DATA_QUALITY>{value}</DATA_QUALITY></metadata>"),
        );
        if value.is_empty() {
            assert_eq!(
                Tdm::from_xml(&xml).unwrap().body.segments[0]
                    .metadata
                    .data_quality,
                Some(TdmDataQuality::Raw)
            );
        } else {
            assert!(Tdm::from_xml(&xml).is_err(), "{value:?}");
        }
    }
}

#[test]
fn xml_unit_bearing_observations_follow_double_underflow_mapping() {
    for field in ["ANGLE_1", "ANGLE_2", "RHUMIDITY"] {
        for token in ["1e-999", "-1e-999", "5e-324"] {
            let input = mutated_once(
                XML,
                "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
                &format!("<{field}>{token}</{field}>"),
            );
            let message = Tdm::from_xml(&input).unwrap();
            let value = match &message.body.segments[0].data.observations[0].data {
                TdmObservationData::Angle1(value) | TdmObservationData::Angle2(value) => *value,
                TdmObservationData::Rhumidity(value) => value.value,
                other => panic!("unexpected observation: {other:?}"),
            };
            let expected = token.parse::<f64>().unwrap();
            assert_eq!(value.to_bits(), expected.to_bits());
            assert_eq!(Tdm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
        }
    }
    for field in ["RANGE", "DOPPLER_INSTANTANEOUS"] {
        let input = mutated_once(
            XML,
            "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
            &format!("<{field}>1e-999</{field}>"),
        );
        assert!(Tdm::from_xml(&input).is_err());
    }
}
