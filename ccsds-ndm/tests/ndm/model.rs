use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::{Message, Ndm};
#[test]
fn combined_validation_checks_the_cdm_root_envelope() {
    // `Cdm`'s inherent `validate` shadows the trait method; it must not be weaker.
    let mut cdm =
        ccsds_ndm::messages::cdm::Cdm::from_kvn(include_str!("../../data/kvn/cdm_362.kvn"))
            .unwrap();
    cdm.id = Some("NOT_A_CDM".into());

    cdm.validate().expect_err("bad root id must be rejected");

    let combined = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![Message::Cdm(cdm)],
    };
    ccsds_ndm::Validate::validate(&combined)
        .expect_err("combined envelope must reject a malformed CDM root");
}
