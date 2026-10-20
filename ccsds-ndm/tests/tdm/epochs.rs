use crate::common::mutated;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::Ndm;

const TDM_KVN: &str = include_str!("../../data/kvn/tdm_e1.kvn");
const TDM_XML: &str = include_str!("../../data/xml/tdm_e21.xml");

#[test]
fn observation_epochs_share_the_global_sixty_four_byte_storage_limit() {
    let baseline = Tdm::from_kvn(TDM_KVN).unwrap();
    let prefix = "2005-159T17:41:00.";
    let epoch = format!("{prefix}{}", "0".repeat(64 - prefix.len()));
    for source in [baseline.to_kvn().unwrap(), baseline.to_xml().unwrap()] {
        let source = source.replace("2005-159T17:41:00", &epoch);
        if source.starts_with('<') {
            Tdm::from_xml(&source).unwrap();
            assert!(Tdm::from_xml(&source.replace(&epoch, &format!("{epoch}0"))).is_err());
        } else {
            Tdm::from_kvn(&source).unwrap();
            assert!(Tdm::from_kvn(&source.replace(&epoch, &format!("{epoch}0"))).is_err());
        }
    }
}

#[test]
fn observations_are_ordered_per_keyword_without_reordering_other_keywords() {
    use ccsds_ndm::messages::tdm::{TdmObservation, TdmObservationData};
    let mut message = Tdm::from_kvn(TDM_KVN).unwrap();
    let row = |epoch: &str, data| TdmObservation {
        epoch: epoch.parse().unwrap(),
        data,
    };
    message.body.segments[0].data.observations = vec![
        row("2005-159T17:41:01", TdmObservationData::Range(1.0)),
        row("2005-159T17:41:00", TdmObservationData::DopplerCount(-2.0)),
        row("2005-159T17:41:02", TdmObservationData::Range(2.0)),
    ];
    assert_eq!(Tdm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert_eq!(Tdm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
    for epoch in ["2005-159T17:41:01.0", "2005-159T17:41:00"] {
        let mut invalid = message.clone();
        invalid.body.segments[0].data.observations[2].epoch = epoch.parse().unwrap();
        crate::common::assert_rejects(&invalid, "RANGE");
        let kvn = message
            .to_kvn()
            .unwrap()
            .replace("2005-159T17:41:02", epoch);
        let xml = message
            .to_xml()
            .unwrap()
            .replace("2005-159T17:41:02", epoch);
        assert!(Tdm::from_kvn(&kvn).is_err());
        assert!(Tdm::from_xml(&xml).is_err());
    }
}

#[test]
fn tdm_epoch_fields_require_calendar_or_ordinal_form() {
    Tdm::from_kvn(TDM_KVN).expect("TDM fixture should parse");

    for (needle, replacement) in [
        (
            "CREATION_DATE = 2005-160T20:15:00Z",
            "CREATION_DATE = 123.5",
        ),
        ("2005-159T17:41:00 32023442781.733", "123.5 32023442781.733"),
    ] {
        let invalid = mutated(TDM_KVN, needle, replacement);
        crate::common::assert_invalid_epoch(&Tdm::from_kvn(&invalid).unwrap_err(), "123.5");
    }

    let invalid = mutated(
        TDM_XML,
        "<EPOCH>2007-069T15:22:22.000</EPOCH>",
        "<EPOCH>123.5</EPOCH>",
    );
    crate::common::assert_invalid_epoch(&Tdm::from_xml(&invalid).unwrap_err(), "123.5");
}
