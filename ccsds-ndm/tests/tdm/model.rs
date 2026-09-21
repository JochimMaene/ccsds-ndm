use crate::common::mutated;
use ccsds_ndm::messages::tdm::{Tdm, TdmBody, TdmObservationData};
use ccsds_ndm::{Ndm, Validate};
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
fn tdm_xml_exhaustive_observations() {
    // Exercise XML deserializer for a wide range of observation types
    let xml = r#"<tdm id="CCSDS_TDM_VERS" version="2.0">
  <header>
    <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
    <ORIGINATOR>TEST</ORIGINATOR>
  </header>
  <body>
    <segment>
      <metadata>
        <TIME_SYSTEM>UTC</TIME_SYSTEM>
        <PARTICIPANT_1>P1</PARTICIPANT_1>
      </metadata>
      <data>
        <observation>
          <EPOCH>2023-01-01T00:00:00</EPOCH>
          <ANGLE_1>1.0</ANGLE_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:01:00</EPOCH>
          <ANGLE_2>2.0</ANGLE_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:02:00</EPOCH>
          <CARRIER_POWER>3.0</CARRIER_POWER>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:03:00</EPOCH>
          <CLOCK_BIAS>4.0</CLOCK_BIAS>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:04:00</EPOCH>
          <CLOCK_DRIFT>5.0</CLOCK_DRIFT>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:05:00</EPOCH>
          <DOPPLER_COUNT>6.0</DOPPLER_COUNT>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:06:00</EPOCH>
          <DOPPLER_INSTANTANEOUS>7.0</DOPPLER_INSTANTANEOUS>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:07:00</EPOCH>
          <DOPPLER_INTEGRATED>8.0</DOPPLER_INTEGRATED>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:08:00</EPOCH>
          <DOR>9.0</DOR>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:09:00</EPOCH>
          <MAG>10.0</MAG>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:10:00</EPOCH>
          <PC_N0>11.0</PC_N0>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:11:00</EPOCH>
          <PR_N0>12.0</PR_N0>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:12:00</EPOCH>
          <PRESSURE>13.0</PRESSURE>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:13:00</EPOCH>
          <RANGE>14.0</RANGE>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:14:00</EPOCH>
          <RCS>15.0</RCS>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:15:00</EPOCH>
          <RECEIVE_FREQ>16.0</RECEIVE_FREQ>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:16:00</EPOCH>
          <RECEIVE_FREQ_1>17.0</RECEIVE_FREQ_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:17:00</EPOCH>
          <RECEIVE_FREQ_2>18.0</RECEIVE_FREQ_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:18:00</EPOCH>
          <RECEIVE_FREQ_3>19.0</RECEIVE_FREQ_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:19:00</EPOCH>
          <RECEIVE_FREQ_4>20.0</RECEIVE_FREQ_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:20:00</EPOCH>
          <RECEIVE_FREQ_5>21.0</RECEIVE_FREQ_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:21:00</EPOCH>
          <RECEIVE_PHASE_CT_1>22.0</RECEIVE_PHASE_CT_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:22:00</EPOCH>
          <RECEIVE_PHASE_CT_2>23.0</RECEIVE_PHASE_CT_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:23:00</EPOCH>
          <RECEIVE_PHASE_CT_3>24.0</RECEIVE_PHASE_CT_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:24:00</EPOCH>
          <RECEIVE_PHASE_CT_4>25.0</RECEIVE_PHASE_CT_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:25:00</EPOCH>
          <RECEIVE_PHASE_CT_5>26.0</RECEIVE_PHASE_CT_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:26:00</EPOCH>
          <RHUMIDITY>27.0</RHUMIDITY>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:27:00</EPOCH>
          <STEC>28.0</STEC>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:28:00</EPOCH>
          <TEMPERATURE>29.0</TEMPERATURE>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:29:00</EPOCH>
          <TRANSMIT_FREQ_1>30.0</TRANSMIT_FREQ_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:30:00</EPOCH>
          <TRANSMIT_FREQ_2>31.0</TRANSMIT_FREQ_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:31:00</EPOCH>
          <TRANSMIT_FREQ_3>32.0</TRANSMIT_FREQ_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:32:00</EPOCH>
          <TRANSMIT_FREQ_4>33.0</TRANSMIT_FREQ_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:33:00</EPOCH>
          <TRANSMIT_FREQ_5>34.0</TRANSMIT_FREQ_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:34:00</EPOCH>
          <TRANSMIT_FREQ_RATE_1>35.0</TRANSMIT_FREQ_RATE_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:35:00</EPOCH>
          <TRANSMIT_FREQ_RATE_2>36.0</TRANSMIT_FREQ_RATE_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:36:00</EPOCH>
          <TRANSMIT_FREQ_RATE_3>37.0</TRANSMIT_FREQ_RATE_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:37:00</EPOCH>
          <TRANSMIT_FREQ_RATE_4>38.0</TRANSMIT_FREQ_RATE_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:38:00</EPOCH>
          <TRANSMIT_FREQ_RATE_5>39.0</TRANSMIT_FREQ_RATE_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:39:00</EPOCH>
          <TRANSMIT_PHASE_CT_1>40.0</TRANSMIT_PHASE_CT_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:40:00</EPOCH>
          <TRANSMIT_PHASE_CT_2>41.0</TRANSMIT_PHASE_CT_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:41:00</EPOCH>
          <TRANSMIT_PHASE_CT_3>42.0</TRANSMIT_PHASE_CT_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:42:00</EPOCH>
          <TRANSMIT_PHASE_CT_4>43.0</TRANSMIT_PHASE_CT_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:43:00</EPOCH>
          <TRANSMIT_PHASE_CT_5>44.0</TRANSMIT_PHASE_CT_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:44:00</EPOCH>
          <TROPO_DRY>45.0</TROPO_DRY>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:45:00</EPOCH>
          <TROPO_WET>46.0</TROPO_WET>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:46:00</EPOCH>
          <VLBI_DELAY>47.0</VLBI_DELAY>
        </observation>
      </data>
    </segment>
  </body>
</tdm>"#;
    let tdm = Tdm::from_xml(xml).expect("parse tdm xml");
    assert_eq!(tdm.body.segments[0].data.observations.len(), 47);
    let obs3 = &tdm.body.segments[0].data.observations[20];
    match obs3.data {
        TdmObservationData::ReceiveFreq5(v) => assert_eq!(v, 21.0),
        _ => panic!("Expected ReceiveFreq5"),
    }

    // Test duplicate EPOCH error handling
    let xml_dup = r#"<tdm id="CCSDS_TDM_VERS" version="2.0">
  <header><CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE><ORIGINATOR>T</ORIGINATOR></header>
  <body><segment><metadata><TIME_SYSTEM>UTC</TIME_SYSTEM><PARTICIPANT_1>P</PARTICIPANT_1></metadata>
  <data><observation><EPOCH>2023-01-01T00:00:00</EPOCH><EPOCH>2023-01-01T00:00:01</EPOCH><RANGE>1.0</RANGE></observation></data>
  </segment></body></tdm>"#;
    crate::common::assert_invalid_format(
        &Tdm::from_xml(xml_dup).unwrap_err(),
        "invalid TDM XML sequence: duplicate or out-of-order child 'EPOCH' in 'observation'",
    );

    // Strict parsing rejects unknown root and observation attributes.
    let xml_unknown = r#"<tdm id="CCSDS_TDM_VERS" version="2.0" extra="val">
  <header><CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE><ORIGINATOR>T</ORIGINATOR></header>
  <body><segment><metadata><TIME_SYSTEM>UTC</TIME_SYSTEM><PARTICIPANT_1>P</PARTICIPANT_1></metadata>
  <data><observation extra="ignore"><EPOCH>2023-01-01T00:00:00</EPOCH><RANGE>1.0</RANGE></observation></data>
  </segment></body></tdm>"#;
    crate::common::assert_invalid_format(
        &Tdm::from_xml(xml_unknown).unwrap_err(),
        "unknown TDM root attribute 'extra'",
    );
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
