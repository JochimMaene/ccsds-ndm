use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::types::CalendarEpoch;
use ccsds_ndm::{Ndm, Validate};

const KVN: &str = include_str!("../data/kvn/aem_g4.kvn");

fn epoch(value: &str) -> CalendarEpoch {
    value.parse().unwrap()
}

fn valid_aem() -> Aem {
    Aem::from_kvn(KVN).unwrap()
}

#[test]
fn aem_enforces_metadata_spans_and_record_order() {
    let mut reversed_span = valid_aem();
    let metadata = &mut reversed_span.body.segment[0].metadata;
    std::mem::swap(&mut metadata.start_time, &mut metadata.stop_time);
    assert!(reversed_span
        .validate()
        .unwrap_err()
        .to_string()
        .contains("START_TIME"));

    let mut repeated_epoch = valid_aem();
    let first_epoch = repeated_epoch.body.segment[0].data.attitude_states[0]
        .quaternion_ephemeris
        .as_ref()
        .unwrap()
        .epoch;
    repeated_epoch.body.segment[0].data.attitude_states[1]
        .quaternion_ephemeris
        .as_mut()
        .unwrap()
        .epoch = first_epoch;
    assert!(repeated_epoch
        .validate()
        .unwrap_err()
        .to_string()
        .contains("strictly increasing"));

    let mut output = Vec::new();
    assert!(repeated_epoch.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty());

    let mut outside_span = valid_aem();
    outside_span.body.segment[0].data.attitude_states[0]
        .quaternion_ephemeris
        .as_mut()
        .unwrap()
        .epoch = epoch("1990-01-01T00:00:00");
    assert!(outside_span
        .validate()
        .unwrap_err()
        .to_string()
        .contains("within START_TIME"));
}

#[test]
fn aem_enforces_useable_span_continuity_between_blocks() {
    let mut message = valid_aem();
    let first = message.body.segment[0].clone();
    message.body.segment = vec![first.clone(), first];
    message.body.segment[1].metadata.useable_start_time =
        message.body.segment[0].metadata.useable_start_time;

    assert!(message
        .validate()
        .unwrap_err()
        .to_string()
        .contains("preceding segment"));
}
