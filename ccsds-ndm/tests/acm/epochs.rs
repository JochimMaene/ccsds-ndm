use crate::common::mutated;
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::{Ndm, Validate};

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

#[test]
fn covariance_times_are_strictly_increasing_on_input_and_after_edits() {
    let source = crate::COV_KVN;
    let original = Acm::from_kvn(source).unwrap();
    for time in ["0.0", "-1.0"] {
        let mut value = original.clone();
        value.body.segment.data.cov[0].cov_lines[1].epoch = time.parse().unwrap();
        crate::common::assert_rejects(&value, "COV time tag");
        assert!(Acm::from_kvn(&mutated(source, "1.096694 6.74", &format!("{time} 6.74"))).is_err());
        let xml = mutated(
            &original.to_xml().unwrap(),
            "<covLine>1.096694 ",
            &format!("<covLine>{time} "),
        );
        assert!(Acm::from_xml(&xml).is_err());
    }
}

#[test]
fn attitude_times_must_be_unique_but_need_not_increase() {
    let original = Acm::from_kvn(include_str!("../../data/kvn/acm_g6.kvn")).unwrap();
    for times in [["0.5", "0.25", "0.0"], ["0.25", "0.0", "0.5"]] {
        let mut value = original.clone();
        for (line, time) in value.body.segment.data.att[0]
            .att_lines
            .iter_mut()
            .zip(times)
        {
            line.epoch = time.parse().unwrap();
        }
        value.validate().unwrap();
        assert_eq!(Acm::from_kvn(&value.to_kvn().unwrap()).unwrap(), value);
        assert_eq!(Acm::from_xml(&value.to_xml().unwrap()).unwrap(), value);
    }
    // Signed zero and trailing zeros do not make a repeated time tag unique.
    for time in ["0.0", "-0.0", "0.250", "+0.25"] {
        let mut value = original.clone();
        value.body.segment.data.att[0].att_lines[2].epoch = time.parse().unwrap();
        crate::common::assert_rejects(&value, "ATT time tag");
    }
    let kvn = mutated(
        include_str!("../../data/kvn/acm_g6.kvn"),
        "0.50 0.73492",
        "0.0 0.73492",
    );
    assert!(Acm::from_kvn(&kvn).is_err());
    let xml = mutated(&original.to_xml().unwrap(), "0.50 0.73492", "0.0 0.73492");
    assert!(Acm::from_xml(&xml).is_err());
}

#[test]
fn history_time_tags_are_relative_or_absolute_one_form_per_block() {
    // ADM 5.3.4.3 permits absolute tags; 5.3.4.5 forbids mixing forms within a block.
    for (source, tokens, element) in [
        (
            include_str!("../../data/kvn/acm_g6.kvn"),
            ["0.0 0.73566", "0.25 0.73529", "0.50 0.73492"],
            "attLine",
        ),
        (
            crate::COV_KVN,
            ["0.0 6.74", "1.096694 6.74", "59.896697 6.74"],
            "covLine",
        ),
    ] {
        let absolute = [
            "2017-12-30T00:00:00.0",
            "2017-12-30T00:00:01Z",
            "2017-364T00:00:02",
        ];
        let mut kvn = source.to_owned();
        for (token, time) in tokens.iter().zip(absolute) {
            let time_token = token.split(' ').next().unwrap();
            kvn = mutated(&kvn, token, &token.replacen(time_token, time, 1));
        }
        let message = Acm::from_kvn(&kvn).unwrap();
        let generated = message.to_kvn().unwrap();
        assert!(generated.contains("\n2017-12-30T00:00:01Z "), "{generated}");
        assert_eq!(Acm::from_kvn(&generated).unwrap(), message);
        let xml = message.to_xml().unwrap();
        crate::common::validate_xml("ACM absolute history time tags", &xml);
        assert!(xml.contains(&format!("<{element}>2017-12-30T00:00:00.0 ")));
        assert_eq!(Acm::from_xml(&xml).unwrap(), message);

        let mixed = mutated(
            source,
            tokens[0],
            &tokens[0].replacen(tokens[0].split(' ').next().unwrap(), absolute[0], 1),
        );
        let error = Acm::from_kvn(&mixed).unwrap_err().to_string();
        assert!(error.contains("ADM 5.3.4.5"), "{error}");
        let mixed_xml = mutated(
            &Acm::from_kvn(source).unwrap().to_xml().unwrap(),
            &format!("<{element}>0.0 "),
            &format!("<{element}>{} ", absolute[0]),
        );
        assert!(Acm::from_xml(&mixed_xml).is_err());
        // Absolute tags are compared within their branch: an earlier covariance tag fails.
        let reversed = mutated(&kvn, absolute[1], "2017-12-29T00:00:00");
        if element == "covLine" {
            assert!(Acm::from_kvn(&reversed).is_err());
        } else {
            Acm::from_kvn(&reversed).unwrap();
        }
    }
    let invalid = mutated(
        include_str!("../../data/kvn/acm_g6.kvn"),
        "0.0 0.73566",
        "2017-13-30T00:00:00 0.73566",
    );
    assert!(Acm::from_kvn(&invalid).is_err());
}

#[test]
fn kvn_none_rate_type_normalizes_to_omission_while_xml_token_is_refused() {
    let source = include_str!("../../data/kvn/acm_g6.kvn");
    let original = Acm::from_kvn(source).unwrap();
    assert!(original.body.segment.data.att[0].rate_type.is_none());
    assert!(Acm::from_xml(&original.to_xml().unwrap())
        .unwrap()
        .body
        .segment
        .data
        .att[0]
        .rate_type
        .is_none());
    let kvn = mutated(
        source,
        "ATT_TYPE = QUATERNION",
        "ATT_TYPE = QUATERNION\nRATE_TYPE = NONE",
    );
    let normalized = Acm::from_kvn(&kvn).unwrap();
    assert_eq!(normalized, original);
    assert!(!normalized.to_kvn().unwrap().contains("RATE_TYPE"));
    assert!(!normalized.to_xml().unwrap().contains("RATE_TYPE"));
    for ending in ["\n", "\r", "\r\n", "\n\r"] {
        for token in ["NONE", "none"] {
            let source = kvn.replace("RATE_TYPE = NONE", &format!("RATE_TYPE = {token}"));
            assert_eq!(
                Acm::from_kvn(&source.replace('\n', ending)).unwrap(),
                original
            );
        }
    }
    assert_eq!(
        Acm::from_kvn(&mutated(&kvn, "RATE_TYPE = NONE", "RATE_TYPE = none")).unwrap(),
        original
    );
    // Blank and `N/A` are the shared KVN spelling of an absent optional value.
    for token in ["", "N/A"] {
        let source = mutated(&kvn, "RATE_TYPE = NONE", &format!("RATE_TYPE = {token}"));
        assert_eq!(Acm::from_kvn(&source).unwrap(), original);
    }
    for token in ["UNKNOWN", "None"] {
        assert!(Acm::from_kvn(&mutated(
            &kvn,
            "RATE_TYPE = NONE",
            &format!("RATE_TYPE = {token}")
        ))
        .is_err());
    }
    let extra_rate_column = mutated(
        &kvn,
        "0.0 0.73566 -0.50547 0.41309 0.180707",
        "0.0 0.73566 -0.50547 0.41309 0.180707 0.1",
    );
    assert!(Acm::from_kvn(&extra_rate_column).is_err());
    let xml = mutated(
        &original.to_xml().unwrap(),
        "</ATT_TYPE>",
        "</ATT_TYPE><RATE_TYPE>NONE</RATE_TYPE>",
    );
    assert!(Acm::from_xml(&xml).is_err());
}

#[test]
fn acm_interval_bounds_cannot_be_reversed() {
    let mut message = Acm::from_kvn(ACM_KVN).unwrap();
    message.body.segment.metadata.start_time = Some("200".parse().unwrap());
    message.body.segment.metadata.stop_time = Some("100".parse().unwrap());
    crate::common::assert_rejects(&message, "STOP_TIME");
    message.body.segment.metadata.stop_time = Some("200".parse().unwrap());
    message.validate().unwrap();
    message.body.segment.metadata.stop_time = Some("2017-12-26T19:40:00".parse().unwrap());
    message.validate().unwrap(); // No conversion between calendar and relative clocks.
    message.body.segment.data.man[0].man_duration = None;
    message.body.segment.data.man[0].man_end_time = Some("50".parse().unwrap());
    crate::common::assert_rejects(&message, "MAN_END_TIME");
    message.body.segment.data.man[0].man_end_time = Some("100".parse().unwrap());
    message.validate().unwrap();
}

#[test]
fn maneuver_kvn_canonicalizes_permissive_relative_numbers() {
    let source = include_str!("../../data/kvn/acm_g7.kvn");
    let input = source.replace("MAN_BEGIN_TIME = 100.0", "MAN_BEGIN_TIME = .5");
    let message = Acm::from_kvn(&input).unwrap();
    let generated = message.to_kvn().unwrap();
    assert!(
        generated.lines().any(|line| line
            .split_once('=')
            .is_some_and(|(key, value)| key.trim() == "MAN_BEGIN_TIME" && value.trim() == "0.5")),
        "{generated}"
    );
    Acm::from_kvn(&generated).unwrap();
    let xml = message.to_xml().unwrap();
    assert!(xml.contains(">.5</MAN_BEGIN_TIME>"));
}
