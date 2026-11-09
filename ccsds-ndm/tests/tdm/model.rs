use ccsds_ndm::messages::tdm::{TdmBody, TdmObservationData};
use ccsds_ndm::Validate;

#[test]
fn doppler_counts_are_integral_and_only_phase_counts_require_their_participant() {
    use ccsds_ndm::messages::tdm::Tdm;
    use ccsds_ndm::Ndm;
    let mut message = Tdm::from_kvn(include_str!("../../data/kvn/tdm_e1.kvn")).unwrap();
    message.body.segments[0].data.observations.truncate(1);
    for count in [-2.0, -0.0, 0.0, 2.0] {
        message.body.segments[0].data.observations[0].data =
            TdmObservationData::DopplerCount(count);
        message.validate().unwrap();
        Tdm::from_kvn(&message.to_kvn().unwrap()).unwrap();
        Tdm::from_xml(&message.to_xml().unwrap()).unwrap();
    }
    message.body.segments[0].data.observations[0].data = TdmObservationData::DopplerCount(0.5);
    crate::common::assert_rejects(&message, "DOPPLER_COUNT");
    // TDM §§3.5.2.8-3.5.2.10 index frequencies by downlink/transmitter, not by participant.
    for key in ["RECEIVE_FREQ_5", "TRANSMIT_FREQ_5", "TRANSMIT_FREQ_RATE_5"] {
        message.body.segments[0].data.observations[0].data =
            TdmObservationData::from_key_val(key, "1").unwrap();
        message.validate().unwrap();
    }
    // §§3.5.2.11-3.5.2.12 tie phase counts to PARTICIPANT_n; an empty XML participant names none.
    for key in ["RECEIVE_PHASE_CT_5", "TRANSMIT_PHASE_CT_5"] {
        message.body.segments[0].data.observations[0].data =
            TdmObservationData::from_key_val(key, "1").unwrap();
        crate::common::assert_rejects(&message, key);
        message.body.segments[0].metadata.participant_5 = Some(" ".into());
        crate::common::assert_rejects(&message, key);
        message.body.segments[0].metadata.participant_5 = Some("receiver-five".into());
        let kvn = message.to_kvn().unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Tdm::from_kvn(&kvn).unwrap(), message);
        assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
        assert!(Tdm::from_kvn(
            &kvn.lines()
                .filter(|line| !line.starts_with("PARTICIPANT_5"))
                .collect::<Vec<_>>()
                .join("\n")
        )
        .is_err());
        assert!(
            Tdm::from_xml(&xml.replace("<PARTICIPANT_5>receiver-five</PARTICIPANT_5>", ""))
                .is_err()
        );
        message.body.segments[0].metadata.participant_5 = None;
    }
}
#[test]
fn tdm_body_requires_segment() {
    let body = TdmBody { segments: vec![] };
    crate::common::assert_validation_field(
        &body.validate().unwrap_err(),
        "segment (at least one required)",
    );
}

#[test]
fn exhaustive_observation_data() {
    // Exercise every variant of TdmObservationData
    use ccsds_ndm::types::Percentage;
    let cases = vec![
        ("ANGLE_1", TdmObservationData::Angle1(45.5)),
        ("ANGLE_2", TdmObservationData::Angle2(30.25)),
        ("CARRIER_POWER", TdmObservationData::CarrierPower(-150.5)),
        ("CLOCK_BIAS", TdmObservationData::ClockBias(0.000001234)),
        ("CLOCK_DRIFT", TdmObservationData::ClockDrift(0.0000000001)),
        ("DOPPLER_COUNT", TdmObservationData::DopplerCount(6.0)),
        (
            "DOPPLER_INSTANTANEOUS",
            TdmObservationData::DopplerInstantaneous(7.0),
        ),
        (
            "DOPPLER_INTEGRATED",
            TdmObservationData::DopplerIntegrated(8.0),
        ),
        ("DOR", TdmObservationData::Dor(9.0)),
        ("MAG", TdmObservationData::Mag(12.5)),
        ("PC_N0", TdmObservationData::PcN0(45.5)),
        ("PR_N0", TdmObservationData::PrN0(35.2)),
        ("PRESSURE", TdmObservationData::Pressure(1013.25)),
        ("RANGE", TdmObservationData::Range(14.0)),
        ("RCS", TdmObservationData::Rcs(1.5)),
        (
            "RECEIVE_FREQ",
            TdmObservationData::ReceiveFreq(8415000000.0),
        ),
        (
            "RECEIVE_FREQ_1",
            TdmObservationData::ReceiveFreq1(8415000001.0),
        ),
        ("RECEIVE_FREQ_2", TdmObservationData::ReceiveFreq2(18.0)),
        ("RECEIVE_FREQ_3", TdmObservationData::ReceiveFreq3(19.0)),
        ("RECEIVE_FREQ_4", TdmObservationData::ReceiveFreq4(20.0)),
        ("RECEIVE_FREQ_5", TdmObservationData::ReceiveFreq5(21.0)),
        (
            "RECEIVE_PHASE_CT_1",
            TdmObservationData::ReceivePhaseCt1("8429753135.986102".into()),
        ),
        (
            "RECEIVE_PHASE_CT_2",
            TdmObservationData::ReceivePhaseCt2("23.0".into()),
        ),
        (
            "RECEIVE_PHASE_CT_3",
            TdmObservationData::ReceivePhaseCt3("24.0".into()),
        ),
        (
            "RECEIVE_PHASE_CT_4",
            TdmObservationData::ReceivePhaseCt4("25.0".into()),
        ),
        (
            "RECEIVE_PHASE_CT_5",
            TdmObservationData::ReceivePhaseCt5("26.0".into()),
        ),
        (
            "RHUMIDITY",
            TdmObservationData::Rhumidity(Percentage::new(65.5, None).unwrap()),
        ),
        ("STEC", TdmObservationData::Stec(50.0)),
        ("TEMPERATURE", TdmObservationData::Temperature(293.15)),
        (
            "TRANSMIT_FREQ_1",
            TdmObservationData::TransmitFreq1(7167941264.0),
        ),
        (
            "TRANSMIT_FREQ_2",
            TdmObservationData::TransmitFreq2(7167941265.0),
        ),
        ("TRANSMIT_FREQ_3", TdmObservationData::TransmitFreq3(31.0)),
        ("TRANSMIT_FREQ_4", TdmObservationData::TransmitFreq4(32.0)),
        ("TRANSMIT_FREQ_5", TdmObservationData::TransmitFreq5(33.0)),
        (
            "TRANSMIT_FREQ_RATE_1",
            TdmObservationData::TransmitFreqRate1(34.0),
        ),
        (
            "TRANSMIT_FREQ_RATE_2",
            TdmObservationData::TransmitFreqRate2(35.0),
        ),
        (
            "TRANSMIT_FREQ_RATE_3",
            TdmObservationData::TransmitFreqRate3(36.0),
        ),
        (
            "TRANSMIT_FREQ_RATE_4",
            TdmObservationData::TransmitFreqRate4(37.0),
        ),
        (
            "TRANSMIT_FREQ_RATE_5",
            TdmObservationData::TransmitFreqRate5(38.0),
        ),
        (
            "TRANSMIT_PHASE_CT_1",
            TdmObservationData::TransmitPhaseCt1("7175173383.615373".into()),
        ),
        (
            "TRANSMIT_PHASE_CT_2",
            TdmObservationData::TransmitPhaseCt2("40.0".into()),
        ),
        (
            "TRANSMIT_PHASE_CT_3",
            TdmObservationData::TransmitPhaseCt3("41.0".into()),
        ),
        (
            "TRANSMIT_PHASE_CT_4",
            TdmObservationData::TransmitPhaseCt4("42.0".into()),
        ),
        (
            "TRANSMIT_PHASE_CT_5",
            TdmObservationData::TransmitPhaseCt5("43.0".into()),
        ),
        ("TROPO_DRY", TdmObservationData::TropoDry(2.3)),
        ("TROPO_WET", TdmObservationData::TropoWet(0.15)),
        ("VLBI_DELAY", TdmObservationData::VlbiDelay(46.0)),
    ];

    let mut kvn_rows = String::new();
    let mut xml_rows = String::new();
    for (expected_key, data) in &cases {
        assert_eq!(data.key(), *expected_key);
        let value = data.value_to_string();
        assert_eq!(
            TdmObservationData::from_key_val(expected_key, &value).unwrap(),
            *data
        );
        kvn_rows.push_str(&format!("{expected_key} = 2023-01-01T00:00:00 {value}\n"));
        xml_rows.push_str(&format!("<observation><EPOCH>2023-01-01T00:00:00</EPOCH><{expected_key}>{value}</{expected_key}></observation>"));
    }
    let kvn = format!("CCSDS_TDM_VERS = 2.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nPARTICIPANT_1 = P1\nPARTICIPANT_2 = P2\nPARTICIPANT_3 = P3\nPARTICIPANT_4 = P4\nPARTICIPANT_5 = P5\nMETA_STOP\nDATA_START\n{kvn_rows}DATA_STOP\n");
    let xml = format!(
        r#"<tdm id="CCSDS_TDM_VERS" version="2.0">
<header><CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE><ORIGINATOR>TEST</ORIGINATOR></header>
<body><segment><metadata><TIME_SYSTEM>UTC</TIME_SYSTEM>
<PARTICIPANT_1>P1</PARTICIPANT_1><PARTICIPANT_2>P2</PARTICIPANT_2><PARTICIPANT_3>P3</PARTICIPANT_3>
<PARTICIPANT_4>P4</PARTICIPANT_4><PARTICIPANT_5>P5</PARTICIPANT_5>
</metadata><data>{xml_rows}</data></segment></body></tdm>"#
    );
    use ccsds_ndm::{messages::tdm::Tdm, Ndm};
    for message in [Tdm::from_kvn(&kvn).unwrap(), Tdm::from_xml(&xml).unwrap()] {
        let observations = &message.body.segments[0].data.observations;
        assert_eq!(observations.len(), cases.len());
        for (observation, (_, expected)) in observations.iter().zip(&cases) {
            assert_eq!(&observation.data, expected);
        }
    }
}

#[test]
fn tdm_observation_data_errors() {
    // Invalid float
    crate::common::assert_invalid_format(
        &TdmObservationData::from_key_val("RANGE", "abc").unwrap_err(),
        "Invalid float: abc",
    );
    // Unknown key
    crate::common::assert_validation_field(
        &TdmObservationData::from_key_val("UNKNOWN", "1.0").unwrap_err(),
        "UNKNOWN",
    );
}

#[test]
fn exact_phase_counts_survive_both_notations_without_floating_point() {
    use ccsds_ndm::messages::tdm::Tdm;
    use ccsds_ndm::{convert, Ndm};
    let baseline = include_str!("../../data/kvn/tdm_e1.kvn");
    for direction in ["RECEIVE", "TRANSMIT"] {
        for index in 1..=5 {
            let key = format!("{direction}_PHASE_CT_{index}");
            for value in [
                "9007199254740993.12345678901234567890",
                "1.234567890123456789e999",
            ] {
                let mut message = Tdm::from_kvn(baseline).unwrap();
                message.body.segments[0].metadata.participant_3 = Some("P3".into());
                message.body.segments[0].metadata.participant_4 = Some("P4".into());
                message.body.segments[0].metadata.participant_5 = Some("P5".into());
                message.body.segments[0].data.observations[0].data =
                    TdmObservationData::from_key_val(&key, value).unwrap();
                let xml = message.to_xml().unwrap();
                assert!(xml.contains(value));
                assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
                // The exponent spelling is XML-only (TDM 4.3.11); see the parsing test.
                if !value.contains('e') {
                    let kvn = message.to_kvn().unwrap();
                    assert!(kvn.contains(value));
                    assert_eq!(Tdm::from_kvn(&kvn).unwrap(), message);
                    assert!(convert(&xml, ccsds_ndm::Notation::Kvn)
                        .unwrap()
                        .contains(value));
                }
                assert_eq!(
                    message.body.segments[0].data.observations[0].data.value(),
                    None
                );
            }
            for invalid in ["NaN", "inf", "", ".", "1e", "1 2", "1\n2"] {
                assert!(
                    TdmObservationData::from_key_val(&key, invalid).is_err(),
                    "{invalid}"
                );
            }
        }
    }
}

#[test]
fn tdm_open_metadata_domains_refuse_only_unrepresentable_xml() {
    use ccsds_ndm::messages::tdm::Tdm;
    use ccsds_ndm::{Ndm, Validate};
    let baseline = include_str!("../../data/kvn/tdm_e1.kvn");
    for (field, value) in [("ANGLE_TYPE", "ICD_CUSTOM"), ("REFERENCE_FRAME", "GCRF")] {
        let mut message = Tdm::from_kvn(baseline).unwrap();
        let metadata = &mut message.body.segments[0].metadata;
        if field == "ANGLE_TYPE" {
            metadata.angle_type = Some(value.parse().unwrap());
        } else {
            metadata.reference_frame = Some(value.parse().unwrap());
        }
        message.validate().unwrap();
        let kvn = message.to_kvn().unwrap();
        assert!(kvn.contains(value));
        assert_eq!(Tdm::from_kvn(&kvn).unwrap(), message);
        assert!(message.to_xml().unwrap_err().to_string().contains(field));
        let mut output = Vec::new();
        assert!(message.write_xml_to(&mut output).is_err());
        assert!(output.is_empty());
    }
    let mut message = Tdm::from_kvn(baseline).unwrap();
    for invalid in ["UNKNOWN", "RANGE,", "COMMENT"] {
        message.body.segments[0].metadata.data_types = Some(invalid.into());
        assert!(message.validate().is_err());
    }
    message.body.segments[0].metadata.data_types = Some("RANGE, TRANSMIT_FREQ_n".into());
    message.validate().unwrap();
}

#[test]
fn dor_and_vlbi_reference_context_is_left_to_the_caller() {
    use ccsds_ndm::messages::tdm::Tdm;
    use ccsds_ndm::Ndm;
    for keyword in ["DOR", "VLBI_DELAY"] {
        let input = format!("CCSDS_TDM_VERS = 2.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nPARTICIPANT_1 = STATION\nMETA_STOP\nDATA_START\n{keyword} = 2023-01-01T00:00:00 1\nDATA_STOP\n");
        let message = Tdm::from_kvn(&input).unwrap();
        message.validate().unwrap();
        assert_eq!(Tdm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
    }
}
