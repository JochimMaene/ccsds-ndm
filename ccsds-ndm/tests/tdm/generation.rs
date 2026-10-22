use crate::common::{assert_rejects, validate_xml};
use crate::KVN;
use ccsds_ndm::messages::tdm::{Tdm, TdmObservationData};
use ccsds_ndm::types::Percentage;
use ccsds_ndm::{Ndm, Validate};
#[test]
fn edited_tdm_metadata_numbers_are_revalidated_before_output() {
    for field in [
        "INTEGRATION_INTERVAL",
        "DOPPLER_COUNT_BIAS",
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
        "FREQ_OFFSET",
        "CORRECTION_ANGLE_1",
        "CORRECTION_ANGLE_2",
        "CORRECTION_DOPPLER",
        "CORRECTION_MAG",
        "CORRECTION_RANGE",
        "CORRECTION_RCS",
        "CORRECTION_RECEIVE",
        "CORRECTION_TRANSMIT",
        "CORRECTION_ABERRATION_YEARLY",
        "CORRECTION_ABERRATION_DIURNAL",
    ] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        let metadata = &mut message.body.segments[0].metadata;
        let target = match field {
            "INTEGRATION_INTERVAL" => &mut metadata.integration_interval,
            "DOPPLER_COUNT_BIAS" => &mut metadata.doppler_count_bias,
            "RANGE_MODULUS" => &mut metadata.range_modulus,
            "TRANSMIT_DELAY_1" => &mut metadata.transmit_delay_1,
            "TRANSMIT_DELAY_2" => &mut metadata.transmit_delay_2,
            "TRANSMIT_DELAY_3" => &mut metadata.transmit_delay_3,
            "TRANSMIT_DELAY_4" => &mut metadata.transmit_delay_4,
            "TRANSMIT_DELAY_5" => &mut metadata.transmit_delay_5,
            "RECEIVE_DELAY_1" => &mut metadata.receive_delay_1,
            "RECEIVE_DELAY_2" => &mut metadata.receive_delay_2,
            "RECEIVE_DELAY_3" => &mut metadata.receive_delay_3,
            "RECEIVE_DELAY_4" => &mut metadata.receive_delay_4,
            "RECEIVE_DELAY_5" => &mut metadata.receive_delay_5,
            "FREQ_OFFSET" => &mut metadata.freq_offset,
            "CORRECTION_ANGLE_1" => &mut metadata.correction_angle_1,
            "CORRECTION_ANGLE_2" => &mut metadata.correction_angle_2,
            "CORRECTION_DOPPLER" => &mut metadata.correction_doppler,
            "CORRECTION_MAG" => &mut metadata.correction_mag,
            "CORRECTION_RANGE" => &mut metadata.correction_range,
            "CORRECTION_RCS" => &mut metadata.correction_rcs,
            "CORRECTION_RECEIVE" => &mut metadata.correction_receive,
            "CORRECTION_TRANSMIT" => &mut metadata.correction_transmit,
            "CORRECTION_ABERRATION_YEARLY" => &mut metadata.correction_aberration_yearly,
            _ => &mut metadata.correction_aberration_diurnal,
        };
        *target = Some(
            if field.starts_with("CORRECTION") || field == "FREQ_OFFSET" {
                f64::NAN
            } else if matches!(field, "INTEGRATION_INTERVAL" | "DOPPLER_COUNT_BIAS") {
                0.0
            } else {
                -1.0
            },
        );
        assert_rejects(&message, field);
    }

    for field in ["INTERPOLATION_DEGREE", "DOPPLER_COUNT_SCALE"] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        let metadata = &mut message.body.segments[0].metadata;
        if field == "INTERPOLATION_DEGREE" {
            metadata.interpolation_degree = Some(0);
        } else {
            metadata.doppler_count_scale = Some(0);
        }
        assert_rejects(&message, field);
    }
}

#[test]
fn tdm_paths_and_single_diff_receive_band_use_segment_context() {
    for path in ["2", "2,4"] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        message.body.segments[0].metadata.path.as_mut().unwrap().0 = path.into();
        assert_rejects(&message, "PATH");
    }

    let single_diff = include_str!("../../data/kvn/tdm_e10.kvn");
    // An empty XML RECEIVE_BAND names no band, so it does not satisfy the rule either.
    for (data, band) in [
        (TdmObservationData::ReceiveFreq(1.0), None),
        (TdmObservationData::Range(1.0), None),
        (TdmObservationData::Range(1.0), Some(String::new())),
    ] {
        let mut message = Tdm::from_kvn(single_diff).unwrap();
        message.body.segments[0].metadata.receive_band = band;
        message.body.segments[0].data.observations.truncate(1);
        message.body.segments[0].data.observations[0].data = data;
        assert_rejects(
            &message,
            "RECEIVE_BAND (required for SINGLE_DIFF frequency or range data)",
        );
    }
    for data in [
        TdmObservationData::ReceiveFreq1(1.0),
        TdmObservationData::Dor(1.0),
        TdmObservationData::VlbiDelay(1.0),
    ] {
        let mut message = Tdm::from_kvn(single_diff).unwrap();
        message.body.segments[0].metadata.receive_band = None;
        message.body.segments[0].data.observations.truncate(1);
        message.body.segments[0].data.observations[0].data = data;
        message.validate().unwrap();
    }
}

#[test]
fn unambiguous_tdm_observation_domains_are_revalidated() {
    for (field, data) in [
        ("TRANSMIT_FREQ_1", TdmObservationData::TransmitFreq1(0.0)),
        ("TROPO_DRY", TdmObservationData::TropoDry(-1.0)),
        ("TROPO_WET", TdmObservationData::TropoWet(-1.0)),
        (
            "RHUMIDITY",
            TdmObservationData::Rhumidity(Percentage {
                value: 101.0,
                units: None,
            }),
        ),
    ] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        message.body.segments[0].data.observations.truncate(1);
        message.body.segments[0].data.observations[0].data = data;
        assert_rejects(&message, field);
    }
}

#[test]
fn tdm_observation_domains_follow_the_narrower_book_range() {
    // TDM 2.0 is narrower than the shared/TDM XSD for these observables; the book rule governs.
    for (field, data) in [
        ("ANGLE_1", TdmObservationData::Angle1(-180.000_000_1)),
        ("ANGLE_1", TdmObservationData::Angle1(360.0)),
        ("ANGLE_2", TdmObservationData::Angle2(-181.0)),
        ("ANGLE_2", TdmObservationData::Angle2(f64::INFINITY)),
        ("RCS", TdmObservationData::Rcs(0.0)),
        ("RCS", TdmObservationData::Rcs(-1.0)),
        ("STEC", TdmObservationData::Stec(0.0)),
        ("STEC", TdmObservationData::Stec(f64::NAN)),
        ("TEMPERATURE", TdmObservationData::Temperature(0.0)),
        ("TEMPERATURE", TdmObservationData::Temperature(-0.5)),
    ] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        message.body.segments[0].data.observations.truncate(1);
        message.body.segments[0].data.observations[0].data = data;
        assert_rejects(&message, field);
    }
}

#[test]
fn tdm_book_narrowed_boundaries_generate_valid_xml() {
    let mut message = Tdm::from_kvn(KVN).unwrap();
    let segment = &mut message.body.segments[0];
    let template = segment.data.observations[0].clone();
    segment.data.observations = [
        TdmObservationData::Angle1(-180.0),
        TdmObservationData::Angle2(359.999_999),
        TdmObservationData::Rcs(f64::MIN_POSITIVE),
        TdmObservationData::Stec(f64::MIN_POSITIVE),
        TdmObservationData::Temperature(f64::MIN_POSITIVE),
    ]
    .into_iter()
    .map(|data| {
        let mut observation = template.clone();
        observation.data = data;
        observation
    })
    .collect();

    message.validate().unwrap();
    let xml = message.to_xml().unwrap();
    validate_xml("TDM book-narrowed boundaries", &xml);
    assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
}

#[test]
fn tdm_revalidates_later_segments_and_observations() {
    let mut message = Tdm::from_kvn(KVN).unwrap();
    message.body.segments.push(message.body.segments[0].clone());
    message.body.segments[1].metadata.integration_interval = Some(0.0);
    assert_rejects(&message, "INTEGRATION_INTERVAL");

    let mut message = Tdm::from_kvn(KVN).unwrap();
    message.body.segments[0].data.observations[1].data = TdmObservationData::TransmitFreq1(0.0);
    assert_rejects(&message, "TRANSMIT_FREQ_1");
}

#[test]
fn unambiguous_tdm_numeric_boundaries_generate_valid_xml() {
    let mut message = Tdm::from_kvn(KVN).unwrap();
    let segment = &mut message.body.segments[0];
    segment.metadata.integration_interval = Some(f64::MIN_POSITIVE);
    segment.metadata.range_modulus = Some(0.0);
    segment.metadata.doppler_count_bias = Some(f64::MIN_POSITIVE);
    segment.metadata.doppler_count_scale = Some(1);
    segment.metadata.transmit_delay_1 = Some(0.0);
    segment.metadata.receive_delay_1 = Some(0.0);
    segment.metadata.interpolation = Some("LINEAR".into());
    segment.metadata.interpolation_degree = Some(1);

    let template = segment.data.observations[0].clone();
    segment.data.observations = [
        TdmObservationData::TransmitFreq1(f64::MIN_POSITIVE),
        TdmObservationData::TropoDry(0.0),
        TdmObservationData::TropoWet(0.0),
        TdmObservationData::Rhumidity(Percentage {
            value: 100.0,
            units: None,
        }),
    ]
    .into_iter()
    .map(|data| {
        let mut observation = template.clone();
        observation.data = data;
        observation
    })
    .collect();

    let xml = message.to_xml().unwrap();
    validate_xml("TDM numeric boundaries", &xml);
    assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
}

#[test]
fn kitchen_sink_roundtrip() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = KITCHEN-SINK-001
META_START
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
RECEIVE_DELAY_1 = 0.000088
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
    let tdm = Tdm::from_kvn(kvn).expect("parse kvn");
    let generated = tdm.to_kvn().expect("generate kvn");
    let tdm2 = Tdm::from_kvn(&generated).expect("parse generated kvn");

    assert_eq!(tdm2, tdm);
}

#[test]
fn xml_sample_parsing() {
    let xml = include_str!("../../data/xml/tdm_e21.xml");
    let mut tdm = Tdm::from_xml(xml).expect("parse xml");
    assert!(!tdm.body.segments.is_empty());

    crate::common::assert_validation_field(&tdm.to_kvn().unwrap_err(), "PARTICIPANT_1");
    tdm.body.segments[0].metadata.participant_1 = "DSS-25".into();
    let generated_kvn = tdm.to_kvn().expect("convert to kvn");
    let tdm2 = Tdm::from_kvn(&generated_kvn).expect("parse generated kvn");

    assert_eq!(tdm2, tdm);
}

#[test]
fn rhumidity_xml_roundtrip_omits_empty_units_attr() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-10
META_STOP
DATA_START
RHUMIDITY = 2023-01-01T00:03:00 12
DATA_STOP
"#;

    let tdm = Tdm::from_kvn(kvn).expect("failed to parse TDM KVN");
    let xml = tdm.to_xml().expect("failed to serialize TDM XML");
    assert!(!xml.contains(r#"RHUMIDITY units="""#));
    assert!(xml.contains("<RHUMIDITY>12</RHUMIDITY>"));

    let parsed = Tdm::from_xml(&xml).expect("failed to parse TDM XML");
    assert_eq!(parsed, tdm);
}

#[test]
fn kvn_numbers_follow_the_digit_limit_without_rounding_phase_counts() {
    let mut message = Tdm::from_kvn(KVN).unwrap();
    let metadata = &mut message.body.segments[0].metadata;
    for field in [
        &mut metadata.integration_interval,
        &mut metadata.freq_offset,
        &mut metadata.range_modulus,
        &mut metadata.doppler_count_bias,
        &mut metadata.transmit_delay_1,
        &mut metadata.transmit_delay_2,
        &mut metadata.transmit_delay_3,
        &mut metadata.transmit_delay_4,
        &mut metadata.transmit_delay_5,
        &mut metadata.receive_delay_1,
        &mut metadata.receive_delay_2,
        &mut metadata.receive_delay_3,
        &mut metadata.receive_delay_4,
        &mut metadata.receive_delay_5,
        &mut metadata.correction_angle_1,
        &mut metadata.correction_angle_2,
        &mut metadata.correction_doppler,
        &mut metadata.correction_mag,
        &mut metadata.correction_range,
        &mut metadata.correction_rcs,
        &mut metadata.correction_receive,
        &mut metadata.correction_transmit,
        &mut metadata.correction_aberration_yearly,
        &mut metadata.correction_aberration_diurnal,
    ] {
        *field = Some(1.234_567_890_123_456_7);
    }
    metadata.corrections_applied = Some(ccsds_ndm::types::YesNo::Yes);
    let observation = message.body.segments[0].data.observations[0].clone();
    message.body.segments[0].data.observations = [
        TdmObservationData::Range(1.234_567_890_123_456_7),
        TdmObservationData::ReceivePhaseCt1("9007199254740993.1234567890123456789".into()),
    ]
    .into_iter()
    .map(|data| {
        let mut observation = observation.clone();
        observation.data = data;
        observation
    })
    .collect();
    let xml = message.to_xml().unwrap();
    assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
    let kvn = message.to_kvn().unwrap();
    assert!(
        !kvn.contains("1.2345678901234567"),
        "unrounded number: {kvn}"
    );
    assert!(kvn.contains("1.234567890123457"));
    let reparsed = Tdm::from_kvn(&kvn).unwrap();
    assert_eq!(
        reparsed.body.segments[0].metadata.freq_offset,
        Some(1.234_567_890_123_457)
    );
    assert_eq!(
        reparsed.body.segments[0].data.observations[0].data.value(),
        Some(1.234_567_890_123_457)
    );
    assert_eq!(
        reparsed.body.segments[0].data.observations[1].data,
        message.body.segments[0].data.observations[1].data
    );
    let mut streamed = Vec::new();
    message.write_kvn_to(&mut streamed).unwrap();
    assert_eq!(streamed, kvn.as_bytes());
}

#[test]
fn kvn_refuses_numbers_whose_rounded_spelling_overflows_before_output() {
    for field in ["FREQ_OFFSET", "RANGE"] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        if field == "FREQ_OFFSET" {
            message.body.segments[0].metadata.freq_offset = Some(f64::MAX);
        } else {
            message.body.segments[0].data.observations[0].data =
                TdmObservationData::Range(f64::MAX);
        }
        message.validate().unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
        assert!(message.to_kvn().unwrap_err().to_string().contains("number"));
        let mut output = Vec::new();
        assert!(message
            .write_kvn_to(&mut output)
            .unwrap_err()
            .to_string()
            .contains("number"));
        assert!(output.is_empty());
    }
}

#[test]
fn kvn_output_refuses_integers_and_lines_beyond_the_kvn_limits() {
    // DOPPLER_COUNT_SCALE is u64 (XSD positiveInteger), but a KVN integer stops at 2^31-1.
    let mut message = Tdm::from_kvn(KVN).unwrap();
    message.body.segments[0].metadata.doppler_count_scale = Some(i32::MAX as u64 + 1);
    message.validate().unwrap();
    Tdm::from_xml(&message.to_xml().unwrap()).unwrap();
    let error = message.to_kvn().unwrap_err();
    assert!(error.to_string().contains("DOPPLER_COUNT_SCALE"), "{error}");

    // A long DATA_TYPES line (Orekit) is read, but TDM 4.2.1 has no continuation lines,
    // so output is refused and names the keyword.
    let types = vec!["RANGE"; 60].join(",");
    let long = KVN.replacen(
        "TIME_SYSTEM = UTC\n",
        &format!("DATA_TYPES = {types}\nTIME_SYSTEM = UTC\n"),
        1,
    );
    let message = Tdm::from_kvn(&long).unwrap();
    Tdm::from_xml(&message.to_xml().unwrap()).unwrap();
    let error = message.to_kvn().unwrap_err();
    assert!(error.to_string().contains("DATA_TYPES"), "{error}");
    assert!(error.to_string().contains("254"), "{error}");
}
