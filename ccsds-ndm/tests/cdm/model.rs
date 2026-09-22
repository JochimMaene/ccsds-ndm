use crate::common::{assert_validation_field, mutated};
use crate::sample_cdm_kvn;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::types::CdmObjectType;
use ccsds_ndm::Ndm;
#[test]
fn cdm_validation_probability_range() {
    // Probability > 1.0
    let mut kvn = sample_cdm_kvn();
    kvn = mutated(
        &kvn,
        "COLLISION_PROBABILITY = 0.001",
        "COLLISION_PROBABILITY = 1.5",
    );
    crate::common::assert_validation_field(&Cdm::from_kvn(&kvn).unwrap_err(), "Probability");
}

#[test]
fn cdm_validation_segment_count_mismatch() {
    let mut cdm = Cdm::from_kvn(&sample_cdm_kvn()).unwrap();

    // Remove one segment => 1 segment
    cdm.body.segments.pop();
    assert_validation_field(&cdm.validate().unwrap_err(), "exactly 2 segments");

    // Add valid segments to check 3 segments (also invalid)
    let seg = cdm.body.segments[0].clone();
    cdm.body.segments.push(seg.clone());
    cdm.body.segments.push(seg); // Now 3
    assert_eq!(cdm.body.segments.len(), 3);
    assert_validation_field(&cdm.validate().unwrap_err(), "exactly 2 segments");
}

#[test]
fn cdm_validation_requires_object1_and_object2() {
    let mut cdm = Cdm::from_kvn(&sample_cdm_kvn()).unwrap();
    cdm.body.segments[1].metadata.object = CdmObjectType::Object1;
    assert_validation_field(
        &cdm.validate().unwrap_err(),
        "exactly one OBJECT1 and one OBJECT2",
    );
}
