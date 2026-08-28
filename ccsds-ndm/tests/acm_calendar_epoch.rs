use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::traits::Ndm;

const ACM_KVN: &str = include_str!("../data/kvn/acm_g7.kvn");

#[test]
fn acm_metadata_reference_epochs_require_calendar_or_ordinal_form() {
    let acm = Acm::from_kvn(ACM_KVN).expect("ACM fixture should parse");
    assert_eq!(
        acm.body.segment.metadata.epoch_tzero.as_str(),
        "2017-12-26T19:40:00.000"
    );

    let invalid_epoch_tzero = ACM_KVN.replace(
        "EPOCH_TZERO = 2017-12-26T19:40:00.000",
        "EPOCH_TZERO = 123.5",
    );
    assert!(
        Acm::from_kvn(&invalid_epoch_tzero).is_err(),
        "accepted numeric ACM EPOCH_TZERO"
    );

    let with_next_leap = ACM_KVN.replace(
        "META_STOP",
        "NEXT_LEAP_EPOCH = 2018-01-01T00:00:00\nMETA_STOP",
    );
    let invalid_next_leap = with_next_leap.replace(
        "NEXT_LEAP_EPOCH = 2018-01-01T00:00:00",
        "NEXT_LEAP_EPOCH = 123.5",
    );
    assert!(
        Acm::from_kvn(&invalid_next_leap).is_err(),
        "accepted numeric ACM NEXT_LEAP_EPOCH"
    );
}

#[test]
fn acm_xml_reference_epochs_require_calendar_or_ordinal_form() {
    let xml = Acm::from_kvn(ACM_KVN)
        .expect("ACM fixture should parse")
        .to_xml()
        .expect("ACM fixture should serialize");

    let invalid_epoch_tzero = xml.replace(
        "<EPOCH_TZERO>2017-12-26T19:40:00.000</EPOCH_TZERO>",
        "<EPOCH_TZERO>123.5</EPOCH_TZERO>",
    );
    assert!(Acm::from_xml(&invalid_epoch_tzero).is_err());

    let with_next_leap = xml.replace(
        "</EPOCH_TZERO>",
        "</EPOCH_TZERO><NEXT_LEAP_EPOCH>2018-01-01T00:00:00</NEXT_LEAP_EPOCH>",
    );
    let invalid_next_leap = with_next_leap.replace(
        "<NEXT_LEAP_EPOCH>2018-01-01T00:00:00</NEXT_LEAP_EPOCH>",
        "<NEXT_LEAP_EPOCH>123.5</NEXT_LEAP_EPOCH>",
    );
    assert!(Acm::from_xml(&invalid_next_leap).is_err());
}

#[test]
fn acm_maneuver_times_use_relative_numeric_form() {
    let numeric = ACM_KVN.replace("MAN_BEGIN_TIME = 100.0", "MAN_BEGIN_TIME = 2.5e+2");
    let numeric = numeric.replace("MAN_DURATION = 450.0", "MAN_END_TIME = 3.0e+2");
    let acm = Acm::from_kvn(&numeric).expect("relative scientific notation should parse");
    let maneuver = &acm.body.segment.data.man[0];
    assert_eq!(maneuver.man_begin_time.as_ref().unwrap().as_str(), "2.5e+2");
    assert_eq!(maneuver.man_end_time.as_ref().unwrap().as_str(), "3.0e+2");

    for replacement in [
        (
            "MAN_BEGIN_TIME = 100.0",
            "MAN_BEGIN_TIME = 2023-01-01T00:00:00",
        ),
        ("MAN_BEGIN_TIME = 100.0", "MAN_BEGIN_TIME = NaN"),
    ] {
        let invalid = ACM_KVN.replace(replacement.0, replacement.1);
        assert!(Acm::from_kvn(&invalid).is_err());
    }
}
