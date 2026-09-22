use crate::common::mutated;
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::Ndm;

const ACM_KVN: &str = include_str!("../../data/kvn/acm_g7.kvn");

#[test]
fn acm_metadata_reference_epochs_require_calendar_or_ordinal_form() {
    let acm = Acm::from_kvn(ACM_KVN).expect("ACM fixture should parse");
    assert_eq!(
        acm.body.segment.metadata.epoch_tzero.as_str(),
        "2017-12-26T19:40:00.000"
    );

    let invalid_epoch_tzero = mutated(
        ACM_KVN,
        "EPOCH_TZERO = 2017-12-26T19:40:00.000",
        "EPOCH_TZERO = 123.5",
    );
    crate::common::assert_invalid_epoch(&Acm::from_kvn(&invalid_epoch_tzero).unwrap_err(), "123.5");

    let with_next_leap = mutated(
        ACM_KVN,
        "META_STOP",
        "NEXT_LEAP_EPOCH = 2018-01-01T00:00:00\nMETA_STOP",
    );
    Acm::from_kvn(&with_next_leap).expect("valid NEXT_LEAP_EPOCH control");
    let invalid_next_leap = mutated(
        &with_next_leap,
        "NEXT_LEAP_EPOCH = 2018-01-01T00:00:00",
        "NEXT_LEAP_EPOCH = 123.5",
    );
    crate::common::assert_invalid_epoch(&Acm::from_kvn(&invalid_next_leap).unwrap_err(), "123.5");
}

#[test]
fn acm_xml_reference_epochs_require_calendar_or_ordinal_form() {
    let xml = Acm::from_kvn(ACM_KVN)
        .expect("ACM fixture should parse")
        .to_xml()
        .expect("ACM fixture should serialize");

    let invalid_epoch_tzero = mutated(
        &xml,
        "<EPOCH_TZERO>2017-12-26T19:40:00.000</EPOCH_TZERO>",
        "<EPOCH_TZERO>123.5</EPOCH_TZERO>",
    );
    crate::common::assert_invalid_epoch(&Acm::from_xml(&invalid_epoch_tzero).unwrap_err(), "123.5");

    let with_next_leap = mutated(
        &xml,
        "</EPOCH_TZERO>",
        "</EPOCH_TZERO><NEXT_LEAP_EPOCH>2018-01-01T00:00:00</NEXT_LEAP_EPOCH>",
    );
    Acm::from_xml(&with_next_leap).expect("valid NEXT_LEAP_EPOCH control");
    let invalid_next_leap = mutated(
        &with_next_leap,
        "<NEXT_LEAP_EPOCH>2018-01-01T00:00:00</NEXT_LEAP_EPOCH>",
        "<NEXT_LEAP_EPOCH>123.5</NEXT_LEAP_EPOCH>",
    );
    crate::common::assert_invalid_epoch(&Acm::from_xml(&invalid_next_leap).unwrap_err(), "123.5");
}

#[test]
fn acm_maneuver_times_use_relative_numeric_form() {
    let numeric = mutated(ACM_KVN, "MAN_BEGIN_TIME = 100.0", "MAN_BEGIN_TIME = 2.5e+2");
    let numeric = mutated(&numeric, "MAN_DURATION = 450.0", "MAN_END_TIME = 3.0e+2");
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
        let invalid = mutated(ACM_KVN, replacement.0, replacement.1);
        crate::common::assert_invalid_epoch(
            &Acm::from_kvn(&invalid).unwrap_err(),
            replacement.1.split_once(" = ").unwrap().1,
        );
    }
}
