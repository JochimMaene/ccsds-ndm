use crate::common::{mutated, mutated_once};
use crate::{KVN, XML};
use ccsds_ndm::error::{CcsdsNdmError, FormatError};
use ccsds_ndm::messages::tdm::{Tdm, TdmObservationData};
use ccsds_ndm::types::{
    TdmAngleType, TdmDataQuality, TdmIntegrationRef, TdmMode, TdmPath, TdmRangeMode, TdmRangeUnits,
    TdmReferenceFrame, TdmTimetagRef, YesNo,
};
use ccsds_ndm::Ndm;
#[test]
fn test_parse_tdm_example_e1_oneway() {
    let kvn = r#"
CCSDS_TDM_VERS = 2.0
COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
COMMENT StarTrek 1-way data, Ka band down
CREATION_DATE = 2005-160T20:15:00Z
ORIGINATOR = NASA
META_START
COMMENT Data quality degraded by antenna pointing problem...
COMMENT Slightly noisy data
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-25
PARTICIPANT_2 = yyyy-nnnA
MODE = SEQUENTIAL
PATH = 2,1
INTEGRATION_INTERVAL = 1
INTEGRATION_REF = MIDDLE
FREQ_OFFSET = 0
TRANSMIT_DELAY_1 = 0.000077
RECEIVE_DELAY_1 = 0.000077
DATA_QUALITY = DEGRADED
META_STOP
DATA_START
COMMENT TRANSMIT_FREQ_2 is spacecraft reference downlink
TRANSMIT_FREQ_2 = 2005-159T17:41:00 32023442781.733
RECEIVE_FREQ_1 = 2005-159T17:41:00 32021034790.7265
RECEIVE_FREQ_1 = 2005-159T17:41:01 32021034828.8432
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(tdm.header.creation_date.to_string(), "2005-160T20:15:00Z");
    assert_eq!(tdm.body.segments.len(), 1);
    let seg = &tdm.body.segments[0];
    assert_eq!(seg.metadata.participant_1, "DSS-25");
    assert_eq!(seg.metadata.participant_2.as_deref(), Some("yyyy-nnnA"));
    assert_eq!(seg.data.observations.len(), 3);
    match &seg.data.observations[0].data {
        TdmObservationData::TransmitFreq2(v) => assert_eq!(*v, 32023442781.733),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn test_parse_tdm_example_e16_optical() {
    let kvn = r#"
CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2012-10-30T20:00:00
ORIGINATOR = ESA
META_START
TIME_SYSTEM = UTC
START_TIME = 2012-10-29T17:46:39.02
STOP_TIME = 2012-10-29T17:50:53.02
PARTICIPANT_1 = TFRM
PARTICIPANT_2 = TRACK_NUMBER_001
MODE = SEQUENTIAL
PATH = 2,1
ANGLE_TYPE = RADEC
REFERENCE_FRAME = EME2000
META_STOP
DATA_START
ANGLE_1 = 2012-10-29T17:46:39.02 332.2298750
ANGLE_2 = 2012-10-29T17:46:39.02 -16.3028389
MAG = 2012-10-29T17:46:39.02 12.1
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    let seg = &tdm.body.segments[0];
    assert_eq!(seg.metadata.angle_type, Some(TdmAngleType::Radec));
    assert_eq!(seg.data.observations.len(), 3);
    match &seg.data.observations[2].data {
        TdmObservationData::Mag(v) => assert_eq!(*v, 12.1),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn test_parse_tdm_example_e18_phase() {
    let kvn = r#"
CCSDS_TDM_VERS=2.0
CREATION_DATE=2005-184T20:15:00
ORIGINATOR=NASA
META_START
TIME_SYSTEM=UTC
PARTICIPANT_1=DSS-55
PARTICIPANT_2=yyyy-nnnA
MODE=SEQUENTIAL
PATH=1,2,1
META_STOP
DATA_START
TRANSMIT_PHASE_CT_1=2005-184T11:12:23 7175173383.615373
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    let seg = &tdm.body.segments[0];
    match &seg.data.observations[0].data {
        TdmObservationData::TransmitPhaseCt1(s) => assert_eq!(*s, 7175173383.615373),
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
            .contexts
            .contains(&"Expected CREATION_DATE"),
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
fn header_optional_comment() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
COMMENT Header comment 1
COMMENT Header comment 2
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(tdm.header.comment.len(), 2);
    assert_eq!(tdm.header.comment[0], "Header comment 1");
}

#[test]
fn header_optional_message_id() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(tdm.header.message_id.as_deref(), Some("MSG-001"));
}

#[test]
fn version_attribute() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(tdm.version, "2.0");
}

#[test]
fn metadata_mandatory_time_system() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    crate::common::assert_validation_field(&Tdm::from_kvn(kvn).unwrap_err(), "TIME_SYSTEM");
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
fn metadata_optional_participants_2_to_5() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
PARTICIPANT_3 = QUASAR_1
PARTICIPANT_4 = RELAY_SAT
PARTICIPANT_5 = DSS-25
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    let seg = &tdm.body.segments[0];
    assert_eq!(seg.metadata.participant_1, "DSS-14");
    assert_eq!(seg.metadata.participant_2.as_deref(), Some("SPACECRAFT_A"));
    assert_eq!(seg.metadata.participant_3.as_deref(), Some("QUASAR_1"));
    assert_eq!(seg.metadata.participant_4.as_deref(), Some("RELAY_SAT"));
    assert_eq!(seg.metadata.participant_5.as_deref(), Some("DSS-25"));
}

#[test]
fn metadata_path_choice() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
MODE = SEQUENTIAL
PATH = 1,2,1
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(
        tdm.body.segments[0].metadata.path,
        Some(TdmPath("1,2,1".to_string()))
    );
    assert!(tdm.body.segments[0].metadata.path_1.is_none());
    assert!(tdm.body.segments[0].metadata.path_2.is_none());
}

#[test]
fn metadata_path_1_path_2_choice() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
PARTICIPANT_3 = DSS-25
MODE = SINGLE_DIFF
PATH_1 = 1,2,1
PATH_2 = 3,2,3
RECEIVE_BAND = X
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 8415000000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    let seg = &tdm.body.segments[0];
    assert!(seg.metadata.path.is_none());
    assert_eq!(seg.metadata.path_1, Some(TdmPath("1,2,1".to_string())));
    assert_eq!(seg.metadata.path_2, Some(TdmPath("3,2,3".to_string())));
}

#[test]
fn metadata_optional_freq_offset() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert!(tdm.body.segments[0].metadata.freq_offset.is_none());
}

#[test]
fn metadata_explicit_freq_offset() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
FREQ_OFFSET = 8415000000.0
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(
        tdm.body.segments[0].metadata.freq_offset,
        Some(8415000000.0)
    );
}

#[test]
fn metadata_range_modulus_default() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert!(tdm.body.segments[0].metadata.range_modulus.is_none());
}

#[test]
fn metadata_data_quality_values() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
DATA_QUALITY = VALIDATED
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(
        tdm.body.segments[0].metadata.data_quality,
        Some(TdmDataQuality::Validated)
    );
}

#[test]
fn metadata_transmit_receive_delays() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
TRANSMIT_DELAY_1 = 0.000077
TRANSMIT_DELAY_2 = 0.000088
RECEIVE_DELAY_1 = 0.000077
RECEIVE_DELAY_2 = 0.000099
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    let seg = &tdm.body.segments[0];
    assert_eq!(seg.metadata.transmit_delay_1, Some(0.000077));
    assert_eq!(seg.metadata.transmit_delay_2, Some(0.000088));
    assert_eq!(seg.metadata.receive_delay_1, Some(0.000077));
    assert_eq!(seg.metadata.receive_delay_2, Some(0.000099));
}

#[test]
fn metadata_turnaround_ratio() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
TURNAROUND_NUMERATOR = 880
TURNAROUND_DENOMINATOR = 749
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    let seg = &tdm.body.segments[0];
    assert_eq!(seg.metadata.turnaround_numerator, Some(880));
    assert_eq!(seg.metadata.turnaround_denominator, Some(749));
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
fn body_multiple_segments() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-25
META_STOP
DATA_START
RANGE = 2023-01-01T01:00:00 2000.0
DATA_STOP
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-34
META_STOP
DATA_START
RANGE = 2023-01-01T02:00:00 3000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(tdm.body.segments.len(), 3);
    assert_eq!(tdm.body.segments[0].metadata.participant_1, "DSS-14");
    assert_eq!(tdm.body.segments[1].metadata.participant_1, "DSS-25");
    assert_eq!(tdm.body.segments[2].metadata.participant_1, "DSS-34");
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
fn data_multiple_observations() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
RANGE = 2023-01-01T00:01:00 1001.0
RANGE = 2023-01-01T00:02:00 1002.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(tdm.body.segments[0].data.observations.len(), 3);
}

#[test]
fn data_comment_optional() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
COMMENT Data section comment
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(tdm.body.segments[0].data.comment.len(), 1);
    assert_eq!(tdm.body.segments[0].data.comment[0], "Data section comment");
}

#[test]
fn observation_angle_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
ANGLE_TYPE = AZEL
META_STOP
DATA_START
ANGLE_1 = 2023-01-01T00:00:00 45.5
ANGLE_2 = 2023-01-01T00:00:00 30.25
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::Angle1(v) => assert_eq!(*v, 45.5),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::Angle2(v) => assert_eq!(*v, 30.25),
        _ => panic!("Wrong type"),
    }
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
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::DopplerInstantaneous(v) => assert_eq!(*v, -0.5),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::DopplerIntegrated(v) => assert_eq!(*v, -0.45),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[2].data {
        TdmObservationData::DopplerCount(v) => assert_eq!(*v, 12345678.0),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn observation_frequency_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 8415000000.0
RECEIVE_FREQ_1 = 2023-01-01T00:00:01 8415000001.0
TRANSMIT_FREQ_1 = 2023-01-01T00:00:02 7167941264.0
TRANSMIT_FREQ_2 = 2023-01-01T00:00:03 7167941265.0
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    assert_eq!(tdm.body.segments[0].data.observations.len(), 4);
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::ReceiveFreq(v) => assert_eq!(*v, 8415000000.0),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn observation_phase_count_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
TRANSMIT_PHASE_CT_1 = 2023-01-01T00:00:00 7175173383.615373
RECEIVE_PHASE_CT_1 = 2023-01-01T00:00:01 8429753135.986102
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::TransmitPhaseCt1(s) => {
            assert_eq!(*s, 7175173383.615373);
        }
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::ReceivePhaseCt1(s) => {
            assert_eq!(*s, 8429753135.986102);
        }
        _ => panic!("Wrong type"),
    }
}

#[test]
fn observation_vlbi_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = DSS-25
MODE = SINGLE_DIFF
PATH_1 = 1,2
PATH_2 = 2,1
META_STOP
DATA_START
DOR = 2023-01-01T00:00:00 0.000123456
VLBI_DELAY = 2023-01-01T00:00:01 -0.000000789
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::Dor(v) => assert_eq!(*v, 0.000123456),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::VlbiDelay(v) => assert_eq!(*v, -0.000000789),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn observation_media_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
STEC = 2023-01-01T00:00:00 50.0
TROPO_DRY = 2023-01-01T00:00:01 2.3
TROPO_WET = 2023-01-01T00:00:02 0.15
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::Stec(v) => assert_eq!(*v, 50.0),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::TropoDry(v) => assert_eq!(*v, 2.3),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[2].data {
        TdmObservationData::TropoWet(v) => assert_eq!(*v, 0.15),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn observation_weather_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
PRESSURE = 2023-01-01T00:00:00 1013.25
RHUMIDITY = 2023-01-01T00:00:01 65.5
TEMPERATURE = 2023-01-01T00:00:02 293.15
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::Pressure(v) => assert_eq!(*v, 1013.25),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::Rhumidity(p) => assert_eq!(p.value, 65.5),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[2].data {
        TdmObservationData::Temperature(v) => assert_eq!(*v, 293.15),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn observation_clock_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
CLOCK_BIAS = 2023-01-01T00:00:00 0.000001234
CLOCK_DRIFT = 2023-01-01T00:00:01 0.0000000001
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::ClockBias(v) => assert_eq!(*v, 0.000001234),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::ClockDrift(v) => assert_eq!(*v, 0.0000000001),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn observation_optical_radar_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
MAG = 2023-01-01T00:00:00 12.5
RCS = 2023-01-01T00:00:01 1.5
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::Mag(v) => assert_eq!(*v, 12.5),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::Rcs(v) => assert_eq!(*v, 1.5),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn observation_signal_strength_types() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
CARRIER_POWER = 2023-01-01T00:00:00 -150.5
PC_N0 = 2023-01-01T00:00:01 45.5
PR_N0 = 2023-01-01T00:00:02 35.2
DATA_STOP
"#;
    let tdm = Tdm::from_kvn(kvn).unwrap();
    match &tdm.body.segments[0].data.observations[0].data {
        TdmObservationData::CarrierPower(v) => assert_eq!(*v, -150.5),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[1].data {
        TdmObservationData::PcN0(v) => assert_eq!(*v, 45.5),
        _ => panic!("Wrong type"),
    }
    match &tdm.body.segments[0].data.observations[2].data {
        TdmObservationData::PrN0(v) => assert_eq!(*v, 35.2),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn all_metadata_optional_fields() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
COMMENT Metadata comment
TRACK_ID = TRACK_001
DATA_TYPES = RANGE,DOPPLER_INTEGRATED
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
MODE = SEQUENTIAL
PATH = 1,2,1
EPHEMERIS_NAME_1 = DSS14_EPHEM
EPHEMERIS_NAME_2 = SC_EPHEM
TRANSMIT_BAND = X
RECEIVE_BAND = X
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
RANGE = 2023-01-01T00:00:00 1000.0
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
    assert_eq!(seg.metadata.receive_band.as_deref(), Some("X"));
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
}

#[test]
fn test_tdm_empty_file_error() {
    let error = Tdm::from_kvn("").unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!(
        parse.message,
        "unterminated or incomplete TDM section sequence"
    );
}

#[test]
fn test_tdm_version_not_first_error() {
    let kvn = r#"
CREATION_DATE = 2023-01-01T00:00:00
CCSDS_TDM_VERS = 2.0
"#;
    let err = Tdm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(err.message.contains("assignment outside a TDM section"));
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected version-not-first error, got: {:?}", err),
    }
}

#[test]
fn test_tdm_unknown_data_keyword_error() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
UNKNOWN_DATA_TYPE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let err = Tdm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("unknown TDM observation keyword")
                        || err.contexts.contains(&"while validating TDM KVN structure")
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected error, got: {:?}", err),
    }
}

#[test]
fn test_tdm_unknown_metadata_key_error() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
UNKNOWN_METADATA = SOME_VALUE
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let err = Tdm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("unknown TDM metadata keyword")
                        || err.contexts.contains(&"while validating TDM KVN structure")
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected error, got: {:?}", err),
    }
}

#[test]
fn later_participant_delays_corrections_and_observations_are_parsed() {
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = P1
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
}

#[test]
fn unknown_header_and_observation_keywords_are_rejected() {
    // Unknown key between header and body should FAIL
    let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
UNKNOWN_KEY = VALUE
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = P1
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let error = Tdm::from_kvn(kvn).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");

    // An unknown observation key must not silently end the data block.
    let kvn_malformed = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = P1
META_STOP
DATA_START
BAD_KEY = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
    let error = Tdm::from_kvn(kvn_malformed).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
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

    let CcsdsNdmError::Validation(error) = Tdm::from_kvn(&bad).unwrap_err() else {
        panic!("expected a validation error");
    };
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
    ] {
        let error = Tdm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

#[test]
fn tdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let epoch = "<EPOCH>2007-069T15:22:22.000</EPOCH>";
    let observable = "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>";
    for (label, source) in [
        (
            "unknown metadata child",
            mutated(XML, "<metadata>", "<metadata><UNKNOWN>1</UNKNOWN>"),
        ),
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
            "duplicate epoch",
            mutated_once(XML, epoch, &format!("{epoch}{epoch}")),
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
fn tdm_xml_accepts_the_schema_defined_optional_observation_units() {
    let angle = mutated_once(
        XML,
        "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
        "<ANGLE_1 units=\"deg\">1.0</ANGLE_1>",
    );
    Tdm::from_xml(&angle).expect("ANGLE_1 units=deg is allowed by the TDM schema");

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
