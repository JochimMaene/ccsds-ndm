use ccsds_ndm::common::AemAttitudeState;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::types::CalendarEpoch;
use ccsds_ndm::{Ndm, Validate};

use crate::KVN;

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
    let AemAttitudeState::QuaternionEphemeris(first) =
        &repeated_epoch.body.segment[0].data.attitude_states[0]
    else {
        unreachable!()
    };
    let first_epoch = first.epoch;
    let AemAttitudeState::QuaternionEphemeris(second) =
        &mut repeated_epoch.body.segment[0].data.attitude_states[1]
    else {
        unreachable!()
    };
    second.epoch = first_epoch;
    assert!(repeated_epoch
        .validate()
        .unwrap_err()
        .to_string()
        .contains("strictly increasing"));

    let mut output = Vec::new();
    assert!(repeated_epoch.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty());

    let mut outside_span = valid_aem();
    let AemAttitudeState::QuaternionEphemeris(state) =
        &mut outside_span.body.segment[0].data.attitude_states[0]
    else {
        unreachable!()
    };
    state.epoch = epoch("1990-01-01T00:00:00");
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

#[test]
fn aem_requires_one_object_but_allows_a_time_system_per_segment() {
    let mut renamed = valid_aem();
    renamed.body.segment[1].metadata.object_name = "PHOBOS SURVEYOR".to_string();
    let error = renamed.validate().unwrap_err().to_string();
    assert!(error.contains("OBJECT_NAME/OBJECT_ID"), "{error}");
    assert!(error.contains("one object throughout the AEM"), "{error}");

    let mut retagged_id = valid_aem();
    retagged_id.body.segment[1].metadata.object_id = "1996-999Z".to_string();
    assert!(retagged_id
        .validate()
        .unwrap_err()
        .to_string()
        .contains("OBJECT_NAME/OBJECT_ID"));

    // ADM fixes the time system per segment, not per message, so segments describing the same
    // object may disagree on TIME_SYSTEM. This guards against importing the OEM restriction.
    let mut mixed_time_systems = valid_aem();
    mixed_time_systems.body.segment[1].metadata.time_system = "TAI".to_string();
    mixed_time_systems.validate().unwrap();
}
