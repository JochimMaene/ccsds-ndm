use ccsds_ndm::messages::tdm::{TdmBody, TdmObservationData};
use ccsds_ndm::Validate;
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
            TdmObservationData::ReceivePhaseCt1(22.0),
        ),
        (
            "RECEIVE_PHASE_CT_2",
            TdmObservationData::ReceivePhaseCt2(23.0),
        ),
        (
            "RECEIVE_PHASE_CT_3",
            TdmObservationData::ReceivePhaseCt3(24.0),
        ),
        (
            "RECEIVE_PHASE_CT_4",
            TdmObservationData::ReceivePhaseCt4(25.0),
        ),
        (
            "RECEIVE_PHASE_CT_5",
            TdmObservationData::ReceivePhaseCt5(26.0),
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
            TdmObservationData::TransmitPhaseCt1(39.0),
        ),
        (
            "TRANSMIT_PHASE_CT_2",
            TdmObservationData::TransmitPhaseCt2(40.0),
        ),
        (
            "TRANSMIT_PHASE_CT_3",
            TdmObservationData::TransmitPhaseCt3(41.0),
        ),
        (
            "TRANSMIT_PHASE_CT_4",
            TdmObservationData::TransmitPhaseCt4(42.0),
        ),
        (
            "TRANSMIT_PHASE_CT_5",
            TdmObservationData::TransmitPhaseCt5(43.0),
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
