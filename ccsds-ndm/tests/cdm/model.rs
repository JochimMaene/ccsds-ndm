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
fn cdm_distance_and_speed_norms_require_nonnegative_scalars() {
    let valid = Cdm::from_kvn(crate::KVN).unwrap();
    for (field, value, unit) in [
        ("MISS_DISTANCE", "715", "m"),
        ("RELATIVE_SPEED", "14762", "m/s"),
    ] {
        let mut edited = valid.clone();
        let target = if field == "MISS_DISTANCE" {
            &mut edited.body.relative_metadata_data.miss_distance.value
        } else {
            &mut edited
                .body
                .relative_metadata_data
                .relative_speed
                .as_mut()
                .unwrap()
                .value
        };
        *target = -1.0;
        crate::common::assert_rejects(&edited, field);
        let invalid_kvn = mutated(
            crate::KVN,
            &format!("{field} = {value} [{unit}]"),
            &format!("{field} = -1 [{unit}]"),
        );
        assert_validation_field(&Cdm::from_kvn(&invalid_kvn).unwrap_err(), field);
        let invalid_xml = mutated(
            crate::XML,
            &format!("<{field} units=\"{unit}\">{value}</{field}>"),
            &format!("<{field} units=\"{unit}\">-1</{field}>"),
        );
        assert_validation_field(&Cdm::from_xml(&invalid_xml).unwrap_err(), field);
    }

    let mut zero = valid;
    zero.body.relative_metadata_data.miss_distance.value = 0.0;
    zero.body
        .relative_metadata_data
        .relative_speed
        .as_mut()
        .unwrap()
        .value = 0.0;
    Cdm::from_kvn(&zero.to_kvn().unwrap()).unwrap();
    Cdm::from_xml(&zero.to_xml().unwrap()).unwrap();
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

#[test]
fn cdm_object_order_frames_and_identifier_repertoire_are_root_checked() {
    use ccsds_ndm::messages::cdm::Cdm;
    use ccsds_ndm::Ndm;
    let valid = Cdm::from_kvn(include_str!("../../data/kvn/cdm_363.kvn")).unwrap();
    let mut reversed = valid.clone();
    reversed.body.segments.swap(0, 1);
    crate::common::assert_rejects(&reversed, "OBJECT");
    let mut different = valid.clone();
    different.body.segments[1].metadata.ref_frame = ccsds_ndm::types::ReferenceFrameType::Gcrf;
    if different.body.segments[0].metadata.ref_frame
        == different.body.segments[1].metadata.ref_frame
    {
        different.body.segments[1].metadata.ref_frame =
            ccsds_ndm::types::ReferenceFrameType::Eme2000;
    }
    crate::common::assert_rejects(&different, "REF_FRAME");
    for field in [
        "ORIGINATOR",
        "MESSAGE_ID",
        "OBJECT_DESIGNATOR",
        "CATALOG_NAME",
        "INTERNATIONAL_DESIGNATOR",
    ] {
        let mut edited = valid.clone();
        let value = "invalid value!".into();
        match field {
            "ORIGINATOR" => edited.header.originator = value,
            "MESSAGE_ID" => edited.header.message_id = value,
            "OBJECT_DESIGNATOR" => edited.body.segments[1].metadata.object_designator = value,
            "CATALOG_NAME" => edited.body.segments[1].metadata.catalog_name = value,
            _ => edited.body.segments[1].metadata.international_designator = value,
        }
        assert!(edited.validate().is_err());
        crate::common::assert_rejects(&edited, field);
    }
}
