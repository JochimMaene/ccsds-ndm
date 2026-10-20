use ccsds_ndm::messages::tdm::{TdmBody, TdmObservationData};
use ccsds_ndm::Validate;

#[test]
fn doppler_counts_are_integral_and_indexed_observations_require_their_participant() {
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
    for key in [
        "RECEIVE_FREQ_5",
        "RECEIVE_PHASE_CT_5",
        "TRANSMIT_FREQ_5",
        "TRANSMIT_FREQ_RATE_5",
        "TRANSMIT_PHASE_CT_5",
    ] {
        message.body.segments[0].data.observations[0].data =
            TdmObservationData::from_key_val(key, "1").unwrap();
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
        ("ANGLE_1", TdmObservationData::Angle1(1.0)),
        ("ANGLE_2", TdmObservationData::Angle2(2.0)),
        ("CARRIER_POWER", TdmObservationData::CarrierPower(3.0)),
        ("CLOCK_BIAS", TdmObservationData::ClockBias(4.0)),
        ("CLOCK_DRIFT", TdmObservationData::ClockDrift(5.0)),
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
        ("MAG", TdmObservationData::Mag(10.0)),
        ("PC_N0", TdmObservationData::PcN0(11.0)),
        ("PR_N0", TdmObservationData::PrN0(12.0)),
        ("PRESSURE", TdmObservationData::Pressure(13.0)),
        ("RANGE", TdmObservationData::Range(14.0)),
        ("RCS", TdmObservationData::Rcs(15.0)),
        ("RECEIVE_FREQ", TdmObservationData::ReceiveFreq(16.0)),
        ("RECEIVE_FREQ_1", TdmObservationData::ReceiveFreq1(17.0)),
        ("RECEIVE_FREQ_2", TdmObservationData::ReceiveFreq2(18.0)),
        ("RECEIVE_FREQ_3", TdmObservationData::ReceiveFreq3(19.0)),
        ("RECEIVE_FREQ_4", TdmObservationData::ReceiveFreq4(20.0)),
        ("RECEIVE_FREQ_5", TdmObservationData::ReceiveFreq5(21.0)),
        (
            "RECEIVE_PHASE_CT_1",
            TdmObservationData::ReceivePhaseCt1("22.0".into()),
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
            TdmObservationData::Rhumidity(Percentage::new(50.0, None).unwrap()),
        ),
        ("STEC", TdmObservationData::Stec(27.0)),
        ("TEMPERATURE", TdmObservationData::Temperature(28.0)),
        ("TRANSMIT_FREQ_1", TdmObservationData::TransmitFreq1(29.0)),
        ("TRANSMIT_FREQ_2", TdmObservationData::TransmitFreq2(30.0)),
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
            TdmObservationData::TransmitPhaseCt1("39.0".into()),
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
        ("TROPO_DRY", TdmObservationData::TropoDry(44.0)),
        ("TROPO_WET", TdmObservationData::TropoWet(45.0)),
        ("VLBI_DELAY", TdmObservationData::VlbiDelay(46.0)),
    ];

    for (expected_key, data) in cases {
        assert_eq!(data.key(), expected_key);
        let val_str = data.value_to_string();
        let parsed = TdmObservationData::from_key_val(expected_key, &val_str).unwrap();
        assert_eq!(data, parsed);
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
                let kvn = message.to_kvn().unwrap();
                let xml = message.to_xml().unwrap();
                assert!(kvn.contains(value));
                assert!(xml.contains(value));
                assert_eq!(Tdm::from_kvn(&kvn).unwrap(), message);
                assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
                assert!(convert(&xml, ccsds_ndm::Notation::Kvn)
                    .unwrap()
                    .contains(value));
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
fn xml_metadata_enumerations_keep_their_case_sensitive_domain() {
    use ccsds_ndm::messages::tdm::Tdm;
    use ccsds_ndm::Ndm;
    let baseline = include_str!("../../data/kvn/tdm_e1.kvn");
    for (field, value) in [("ANGLE_TYPE", "AZEL"), ("REFERENCE_FRAME", "EME2000")] {
        let mut message = Tdm::from_kvn(baseline).unwrap();
        let metadata = &mut message.body.segments[0].metadata;
        if field == "ANGLE_TYPE" {
            metadata.angle_type = Some(value.parse().unwrap());
        } else {
            metadata.reference_frame = Some(value.parse().unwrap());
        }
        let xml = message.to_xml().unwrap();
        let lower = xml.replace(
            &format!("<{field}>{value}"),
            &format!("<{field}>{}", value.to_ascii_lowercase()),
        );
        assert_ne!(xml, lower);
        assert!(Tdm::from_xml(&lower).is_err());
    }
}
