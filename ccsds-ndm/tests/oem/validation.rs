use crate::{KVN_FIXTURES, XML};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::types::Epoch;
use ccsds_ndm::{Ndm, Validate};

fn epoch(value: &str) -> Epoch {
    value.parse().unwrap()
}

#[test]
fn all_segments_must_describe_one_object_and_one_time_system() {
    let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    assert!(message.body.segment.len() > 1);
    message.body.segment[1].metadata.time_system = "TAI".into();
    assert_eq!(
        message.validate().unwrap_err().field_path().as_deref(),
        Some("body.segment[1].metadata.time_system")
    );

    let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    message.body.segment[1].metadata.object_id = "DIFFERENT".into();
    crate::common::assert_validation_field(
        &message.validate().unwrap_err(),
        "OBJECT_NAME/OBJECT_ID",
    );
}

#[test]
fn consecutive_useable_spans_may_touch_but_not_overlap() {
    let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    let second = &mut message.body.segment[1].metadata;
    second.start_time = epoch("2019-12-28T21:00:00.000");
    message
        .validate()
        .expect("total spans may overlap when useable spans do not");

    message.body.segment[1].metadata.useable_start_time = Some(epoch("2019-12-28T21:23:00.331"));
    message
        .validate()
        .expect("a shared useable-span endpoint is allowed");

    message.body.segment[1].metadata.useable_start_time = Some(epoch("2019-12-28T21:22:00.331"));
    let error = message
        .validate()
        .expect_err("consecutive useable spans must not overlap");
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[1].metadata.useable_start_time")
    );
}

#[test]
fn total_spans_may_overlap_when_useable_bounds_are_omitted() {
    // ODM 5.2.4.4 forbids overlap only between USEABLE_STOP_TIME and the next
    // USEABLE_START_TIME; STOP_TIME may exceed the next START_TIME.
    for (omit_stop, omit_start) in [(true, true), (true, false), (false, true)] {
        let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
        if omit_stop {
            message.body.segment[0].metadata.useable_stop_time = None;
        }
        let second = &mut message.body.segment[1].metadata;
        second.start_time = epoch("2019-12-28T21:00:00");
        second.useable_start_time = (!omit_start).then(|| epoch("2019-12-28T21:01:00"));
        message.validate().expect("overlapping total spans");
        assert_eq!(Oem::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
        assert_eq!(Oem::from_xml(&message.to_xml().unwrap()).unwrap(), message);
    }
}

#[test]
fn time_system_comparison_accepts_normative_lowercase_spelling() {
    let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    message.body.segment[1]
        .metadata
        .time_system
        .make_ascii_lowercase();
    message.validate().unwrap();
    assert_eq!(Oem::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
}

#[test]
fn object_identity_compares_ccsds_text_values_without_rewriting_them() {
    let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    message.body.segment[1].metadata.object_name = "MARS_GLOBAL  SURVEYOR".into();
    message.validate().unwrap();
    assert_eq!(Oem::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);

    // ODM 7.5.3 admits the all-lowercase spelling of the same value.
    message.body.segment[1]
        .metadata
        .object_name
        .make_ascii_lowercase();
    message.body.segment[1]
        .metadata
        .object_id
        .make_ascii_lowercase();
    message.validate().unwrap();

    message.body.segment[1].metadata.object_name = "MARS GLOBAL SURVEYOR 2".into();
    crate::common::assert_validation_field(
        &message.validate().unwrap_err(),
        "OBJECT_NAME/OBJECT_ID",
    );
}

#[test]
fn oem_time_tags_are_absolute_and_metadata_ranges_are_consistent() {
    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].data.state_vector[0].epoch = epoch("123.5");
    assert_eq!(
        message.validate().unwrap_err().field_path().as_deref(),
        Some("body.segment[0].data.state_vector[0].epoch")
    );

    let mut message = Oem::from_xml(XML).unwrap();
    let metadata = &mut message.body.segment[0].metadata;
    std::mem::swap(&mut metadata.start_time, &mut metadata.stop_time);
    let error = message.validate().unwrap_err();
    assert_eq!(error.code(), Some("validation.invalid_value"));
    crate::common::assert_validation_field(&error, "START_TIME/STOP_TIME");

    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].metadata.useable_start_time = Some(epoch("2019-12-01T00:00:00"));
    crate::common::assert_validation_field(&message.validate().unwrap_err(), "USEABLE_START_TIME");

    let mut message = Oem::from_xml(XML).unwrap();
    let metadata = &mut message.body.segment[0].metadata;
    std::mem::swap(
        &mut metadata.useable_start_time,
        &mut metadata.useable_stop_time,
    );
    crate::common::assert_validation_field(
        &message.validate().unwrap_err(),
        "USEABLE_START_TIME/USEABLE_STOP_TIME",
    );
}

#[test]
fn ephemeris_records_are_in_span_but_need_not_be_ordered() {
    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].data.state_vector[0].epoch = epoch("2019-12-01T00:00:00");
    crate::common::assert_validation_field(&message.validate().unwrap_err(), "stateVector EPOCH");

    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].data.state_vector[2].epoch = epoch("2019-12-18T12:00:30.331");
    message
        .validate()
        .expect("OEM does not require ephemeris records to be ordered");
}

#[test]
fn covariance_epochs_are_not_bounded_by_the_total_span() {
    // Table 5-3 says the total span covers the covariance data, but example G-14 (the XML
    // fixture) has its covariance epoch after STOP_TIME. With the book in conflict, the
    // covariance epochs are left unbounded; state epochs stay bounded.
    let valid = Oem::from_xml(XML).unwrap();
    let metadata = &valid.body.segment[0].metadata;
    let covariance = &valid.body.segment[0].data.covariance_matrix[0];
    assert!(covariance.epoch.to_string() > metadata.stop_time.to_string());
    for value in ["2019-12-18T11:59:59", "2019-12-29T00:00:00"] {
        let mut message = valid.clone();
        message.body.segment[0].data.covariance_matrix[0].epoch = epoch(value);
        message.validate().unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Oem::from_kvn(&kvn).unwrap(), message);
        message.to_xml().unwrap();
    }
}

#[test]
fn covariance_time_tags_are_ordered_by_increasing_time() {
    // ODM 5.2.5.7 orders matrices by increasing time tag; it does not exclude equal tags.
    let mut message = Oem::from_xml(XML).unwrap();
    let mut covariance = message.body.segment[0].data.covariance_matrix[0].clone();
    covariance.cov_ref_frame = Some("RTN".into());
    message.body.segment[0]
        .data
        .covariance_matrix
        .push(covariance.clone());
    message.validate().unwrap();

    covariance.epoch = message.body.segment[0].metadata.start_time;
    message.body.segment[0]
        .data
        .covariance_matrix
        .push(covariance);
    let error = message.validate().unwrap_err();
    crate::common::assert_validation_field(&error, "covarianceMatrix EPOCH");
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[0].data.covariance_matrix[2].epoch")
    );
}

#[test]
fn elapsed_time_systems_use_three_digit_day_durations() {
    // ODM 3.2.3.2: MET and MRT times are durations from an epoch in three-digit days.
    let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    message.body.segment.truncate(1);
    let segment = &mut message.body.segment[0];
    segment.metadata.time_system = "MET".into();
    segment.metadata.start_time = epoch("0000-000T00:00:00");
    segment.metadata.useable_start_time = None;
    segment.metadata.useable_stop_time = None;
    segment.metadata.stop_time = epoch("0000-400T00:00:00");
    segment.data.state_vector.truncate(2);
    segment.data.state_vector[0].epoch = epoch("0000-000T00:10:00");
    segment.data.state_vector[1].epoch = epoch("0000-399T23:59:59.5");
    message.validate().unwrap();
    assert_eq!(Oem::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);

    // 7.5.11: REF_FRAME_EPOCH is also interpreted in TIME_SYSTEM.
    message.body.segment[0].metadata.ref_frame_epoch = Some(epoch("0000-000T00:00:00"));
    message.validate().unwrap();
    assert_eq!(Oem::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert_eq!(Oem::from_xml(&message.to_xml().unwrap()).unwrap(), message);

    message.body.segment[0].data.state_vector[1].epoch = epoch("0000-400T00:00:01");
    crate::common::assert_validation_field(&message.validate().unwrap_err(), "stateVector EPOCH");
    message.body.segment[0].data.state_vector[1].epoch = epoch("0000-399T00:00:00");
    message.body.segment[0].metadata.time_system = "UTC".into();
    crate::common::assert_validation_field(&message.validate().unwrap_err(), "START_TIME");
    message.body.segment[0].metadata.start_time = epoch("2020-001T00:00:00");
    message.body.segment[0].metadata.stop_time = epoch("2020-366T00:00:00");
    for state in &mut message.body.segment[0].data.state_vector {
        state.epoch = epoch("2020-002T00:00:00");
    }
    crate::common::assert_validation_field(&message.validate().unwrap_err(), "REF_FRAME_EPOCH");
}

#[test]
fn kvn_values_listed_in_the_book_use_a_single_case() {
    // ODM 7.5.3 is a KVN rule for normative values: those listed in 3.2.3.2 and 3.2.3.3. XML
    // text follows xsd:string (8.13.5), which the schema leaves unrestricted here.
    for (from, to, field) in [
        ("TIME_SYSTEM = UTC", "TIME_SYSTEM = Utc", "TIME_SYSTEM"),
        ("REF_FRAME = EME2000", "REF_FRAME = Eme2000", "REF_FRAME"),
    ] {
        let kvn = crate::common::mutated_once(KVN_FIXTURES[0], from, to);
        crate::common::assert_validation_field(&Oem::from_kvn(&kvn).unwrap_err(), field);
    }

    // ICD-defined values and registry centers keep their spelling in KVN.
    let custom = crate::common::mutated_once(
        KVN_FIXTURES[0],
        "REF_FRAME = EME2000",
        "REF_FRAME = MissionFrame",
    );
    let custom = crate::common::mutated_once(
        &custom,
        "CENTER_NAME = MARS BARYCENTER",
        "CENTER_NAME = Mars Barycenter",
    );
    Oem::from_kvn(&custom).unwrap();

    // The same spellings are valid XML; only conversion to KVN refuses them.
    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].metadata.time_system = "Utc".into();
    message.body.segment[0].data.covariance_matrix[0].cov_ref_frame = Some("Icrf".into());
    message.validate().unwrap();
    let reparsed = Oem::from_xml(&message.to_xml().unwrap()).unwrap();
    assert_eq!(reparsed, message);
    crate::common::assert_validation_field(&message.to_kvn().unwrap_err(), "TIME_SYSTEM");
    message.body.segment[0].metadata.time_system = "UTC".into();
    crate::common::assert_validation_field(&message.to_kvn().unwrap_err(), "COV_REF_FRAME");
}

#[test]
fn xml_values_follow_xsd_double_but_kvn_needs_finite_numbers() {
    // ODM 8.13.4 gives XML numbers the xsd:double conventions, including INF and NaN; KVN
    // numbers (7.5.5-7.5.7) are finite.
    let source = crate::common::mutated_once(XML, "<X>2789.6</X>", "<X>NaN</X>");
    let source =
        crate::common::mutated_once(&source, "<Y_DOT>-2.50</Y_DOT>", "<Y_DOT>-INF</Y_DOT>");
    let source = crate::common::mutated_once(
        &source,
        "<CX_X>0.316</CX_X>",
        "<CX_X><![CDATA[INF]]></CX_X>",
    );
    let message = Oem::from_xml(&source).unwrap();
    assert!(message.body.segment[0].data.state_vector[0]
        .x
        .value
        .is_nan());
    assert_eq!(
        message.body.segment[0].data.covariance_matrix[0].cx_x.value,
        f64::INFINITY
    );
    let xml = message.to_xml().unwrap();
    crate::common::validate_xml("non-finite OEM values", &xml);
    let reparsed = Oem::from_xml(&xml).unwrap();
    assert!(reparsed.body.segment[0].data.state_vector[0]
        .x
        .value
        .is_nan());
    let error = message.to_kvn().unwrap_err();
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[0].data.state_vector[0].x")
    );

    // XSD 1.0 spells the special values only INF, -INF, and NaN, however the text is encoded.
    for spelling in [
        "inf",
        "+INF",
        "nan",
        "Infinity",
        "<![CDATA[inf]]>",
        "i&#110;f",
    ] {
        let invalid =
            crate::common::mutated_once(XML, "<X>2789.6</X>", &format!("<X>{spelling}</X>"));
        let error = Oem::from_xml(&invalid).unwrap_err();
        assert_eq!(
            error.code(),
            Some("parse.xml.syntax"),
            "{spelling}: {error}"
        );
        assert!(
            error.to_string().contains("is not an xsd:double value"),
            "{spelling}: {error}"
        );
    }
    let comment = crate::common::mutated_once(
        XML,
        "<COMMENT>OEM WITH OPTIONAL ACCELERATIONS</COMMENT>",
        "<COMMENT>nan</COMMENT>",
    );
    Oem::from_xml(&comment).unwrap();
}

#[test]
fn epoch_fractions_stop_at_the_fixed_point_maximum() {
    // ODM 7.5.10: fractional seconds may use up to the 16 digits of a fixed-point number.
    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].data.state_vector[1].epoch =
        epoch("2019-12-18T12:01:00.3310000000000000Z");
    message.validate().unwrap();
    message.body.segment[0].data.state_vector[1].epoch =
        epoch("2019-12-18T12:01:00.33100000000000000Z");
    crate::common::assert_validation_field(&message.validate().unwrap_err(), "stateVector EPOCH");

    let mut message = Oem::from_xml(XML).unwrap();
    message.header.creation_date = "2019-11-04T17:22:31.3310000000000000".parse().unwrap();
    message.validate().unwrap();
    message.header.creation_date = "2019-11-04T17:22:31.33100000000000000".parse().unwrap();
    crate::common::assert_validation_field(&message.validate().unwrap_err(), "CREATION_DATE");
}

#[test]
fn padded_time_systems_denote_the_same_time_system() {
    // XML keeps a padded xsd:string as written (8.13.5); the time system it names is read
    // without the padding, which the model preserves.
    let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    message.body.segment[1].metadata.time_system = " UTC\n".into();
    message.validate().unwrap();
    let reparsed = Oem::from_xml(&message.to_xml().unwrap()).unwrap();
    assert_eq!(reparsed.body.segment[1].metadata.time_system, " UTC\n");

    // A padded elapsed time system still selects duration epochs (3.2.3.2).
    message.body.segment.truncate(1);
    let metadata = &mut message.body.segment[0].metadata;
    metadata.time_system = " MET ".into();
    metadata.start_time = epoch("0000-000T00:00:00");
    metadata.useable_start_time = None;
    metadata.useable_stop_time = None;
    metadata.stop_time = epoch("0000-400T00:00:00");
    message.body.segment[0].data.state_vector.truncate(1);
    message.body.segment[0].data.state_vector[0].epoch = epoch("0000-000T00:10:00");
    message.validate().unwrap();
}

#[test]
fn metadata_errors_point_at_real_fields() {
    let base = || Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    let path = |message: Oem| message.validate().unwrap_err().field_path();
    let prefix = "body.segment[0].metadata";

    let mut message = base();
    message.body.segment[0].metadata.interpolation_degree = None;
    assert_eq!(
        path(message).unwrap(),
        format!("{prefix}.interpolation_degree")
    );

    let mut message = base();
    let metadata = &mut message.body.segment[0].metadata;
    std::mem::swap(&mut metadata.start_time, &mut metadata.stop_time);
    assert_eq!(path(message).unwrap(), format!("{prefix}.start_time"));

    let mut message = base();
    let metadata = &mut message.body.segment[0].metadata;
    std::mem::swap(
        &mut metadata.useable_start_time,
        &mut metadata.useable_stop_time,
    );
    assert_eq!(
        path(message).unwrap(),
        format!("{prefix}.useable_start_time")
    );

    let mut message = base();
    message.body.segment[0].metadata.useable_stop_time = Some(epoch("2030-01-01T00:00:00"));
    assert_eq!(
        path(message).unwrap(),
        format!("{prefix}.useable_stop_time")
    );

    let mut message = base();
    message.body.segment[0].metadata.center_name = " ".into();
    assert_eq!(path(message).unwrap(), format!("{prefix}.center_name"));

    let mut message = base();
    message.body.segment[0].metadata.ref_frame_epoch = Some(epoch("123.5"));
    assert_eq!(path(message).unwrap(), format!("{prefix}.ref_frame_epoch"));
}

#[test]
fn useable_spans_may_not_overlap_but_need_not_be_in_time_order() {
    // ODM 5.2.4.4 forbids overlap between consecutive useable spans, except at a shared
    // endpoint; it does not order the segments.
    // Total spans wide enough for both segments' states, so only the useable spans matter.
    let set = |message: &mut Oem, index: usize, start: &str, stop: &str| {
        let metadata = &mut message.body.segment[index].metadata;
        metadata.start_time = epoch("2019-12-18T00:00:00");
        metadata.stop_time = epoch("2019-12-31T00:00:00");
        metadata.useable_start_time = Some(epoch(start));
        metadata.useable_stop_time = Some(epoch(stop));
    };
    let mut reversed = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    reversed.body.segment.swap(0, 1);
    reversed
        .validate()
        .expect("disjoint segments in reverse time order");

    for (first, second) in [
        (
            ("2019-12-18T12:10:00.331", "2019-12-28T22:10:00"),
            ("2019-12-28T22:08:02.5", "2019-12-30T01:18:02.5"),
        ),
        (
            ("2019-12-28T22:08:02.5", "2019-12-30T01:18:02.5"),
            ("2019-12-18T12:10:00.331", "2019-12-28T22:10:00"),
        ),
    ] {
        let mut message = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
        if first.0 > second.0 {
            message.body.segment.swap(0, 1);
        }
        set(&mut message, 0, first.0, first.1);
        set(&mut message, 1, second.0, second.1);
        let error = message.validate().unwrap_err();
        crate::common::assert_validation_field(&error, "USEABLE_START_TIME");
        assert_eq!(
            error.field_path().as_deref(),
            Some("body.segment[1].metadata.useable_start_time")
        );
    }

    let mut shared = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    set(
        &mut shared,
        0,
        "2019-12-18T12:10:00.331",
        "2019-12-28T22:08:02.5",
    );
    shared.validate().expect("shared endpoint");

    // Without both bounds a span is not known, so no overlap is claimed.
    let mut partial = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    set(
        &mut partial,
        0,
        "2019-12-18T12:10:00.331",
        "2019-12-29T00:00:00",
    );
    partial.body.segment[1].metadata.useable_stop_time = None;
    partial.validate().expect("incomplete span");
}
