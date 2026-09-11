use crate::common::{assert_rejects, validate_xml};
use crate::{KVN_FIXTURES, XML};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::{Message, Ndm};

#[test]
fn every_shipped_fixture_generates_deterministic_xsd_valid_xml_and_reparseable_kvn() {
    let mut messages = Vec::new();
    for (index, source) in KVN_FIXTURES.into_iter().enumerate() {
        messages.push((
            format!("oem_g{}.kvn", index + 11),
            Oem::from_kvn(source).unwrap(),
        ));
    }
    messages.push(("oem_g14.xml".into(), Oem::from_xml(XML).unwrap()));

    for (name, message) in messages {
        let xml = message.to_xml().unwrap();
        assert_eq!(message.to_xml().unwrap(), xml);
        validate_xml(&name, &xml);
        assert_eq!(Oem::from_xml(&xml).unwrap(), message);

        let kvn = message.to_kvn().unwrap();
        assert!(kvn.lines().all(|line| {
            line.len() <= 254 && line.bytes().all(|byte| (b' '..=b'~').contains(&byte))
        }));
        assert_eq!(Oem::from_kvn(&kvn).unwrap(), message);
    }
}

#[test]
fn generation_diagnostics_identify_the_message_and_field() {
    let message = Oem::from_kvn(KVN_FIXTURES[2]).unwrap();
    let mut invalid = message;
    invalid.body.segment[0].metadata.object_name.clear();
    for error in [
        invalid.to_kvn().unwrap_err(),
        invalid.to_xml().unwrap_err(),
        Message::Oem(invalid.clone()).to_kvn().unwrap_err(),
        Message::Oem(invalid).to_xml().unwrap_err(),
    ] {
        assert_eq!(error.code(), Some("validation.missing_required_field"));
        assert_eq!(
            error.field_path().as_deref(),
            Some("body.segment[0].metadata.object_name")
        );
        let diagnostic = error
            .diagnostic()
            .expect("generation context should be present");
        assert_eq!(
            diagnostic.message_kind,
            ccsds_ndm::validation::MessageKind::Oem
        );
        assert_eq!(diagnostic.source_edition, Some("3.0"));
    }
}

#[test]
fn kvn_generation_rejects_overlapping_useable_spans() {
    let mut overlapping_useable_spans = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    overlapping_useable_spans.body.segment[1]
        .metadata
        .start_time = "2019-12-28T21:00:00.000".parse().unwrap();
    overlapping_useable_spans.body.segment[1]
        .metadata
        .useable_start_time = Some("2019-12-28T21:22:00.331".parse().unwrap());

    assert_rejects(&overlapping_useable_spans, "USEABLE_START_TIME");
}

#[test]
fn kvn_rounds_to_the_ccsds_digit_limit_and_rejects_partial_acceleration() {
    let mut message = Oem::from_kvn(KVN_FIXTURES[2]).unwrap();
    message.body.segment[0].data.state_vector[0].x.value = 1.234_567_890_123_456_7;
    assert!(message.to_kvn().unwrap().contains("1.234567890123457e0"));
    message.to_xml().expect("XML retains the exact f64 value");

    let mut message = Oem::from_xml(XML).unwrap();
    let state = &mut message.body.segment[0].data.state_vector[0];
    state.y_ddot = None;
    let mut output = Vec::new();
    message
        .write_kvn_to(&mut output)
        .expect_err("partial acceleration must fail preflight");
    assert!(output.is_empty());
    message
        .to_xml()
        .expect("partial acceleration remains valid XML");
}

#[test]
fn kvn_rejects_an_overlong_raw_record_before_writing() {
    let mut message = Oem::from_xml(XML).unwrap();
    let state = &mut message.body.segment[0].data.state_vector[0];
    state.epoch = "2019-12-18T12:00:00.1111111111111111111111111111111111111111"
        .parse()
        .unwrap();
    for value in [
        &mut state.x.value,
        &mut state.y.value,
        &mut state.z.value,
        &mut state.x_dot.value,
        &mut state.y_dot.value,
        &mut state.z_dot.value,
        &mut state.x_ddot.as_mut().unwrap().value,
        &mut state.y_ddot.as_mut().unwrap().value,
        &mut state.z_ddot.as_mut().unwrap().value,
    ] {
        *value = 1.797_693_134_862_315e308;
    }

    let mut output = Vec::new();
    let error = message
        .write_kvn_to(&mut output)
        .expect_err("a record over 254 characters must fail preflight");
    assert_eq!(error.code(), Some("validation.out_of_range"));
    assert!(output.is_empty());
}

#[test]
fn unsupported_editions_and_edition_specific_fields_are_rejected() {
    let mut historical = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    historical.version = "1.0".into();
    assert!(historical.to_kvn().is_err());
    assert!(historical.to_xml().is_err());

    let mut version_two = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    version_two.version = "2.0".into();
    version_two.header.message_id = Some("OEM-3-ONLY".into());
    assert!(version_two.to_kvn().is_err());
}

#[test]
fn notation_specific_text_rules_fail_before_output() {
    let mut message = Oem::from_kvn(KVN_FIXTURES[2]).unwrap();
    message.body.segment[0].metadata.object_name = "MARS €".into();
    message
        .to_xml()
        .expect("Unicode text is representable in XML 1.0");
    assert_eq!(
        message.to_kvn().unwrap_err().code(),
        Some("validation.invalid_value")
    );

    message.body.segment[0].metadata.object_name = "MARS\u{1}".into();
    assert_eq!(
        message.to_xml().unwrap_err().code(),
        Some("validation.invalid_value")
    );

    message.body.segment[0].metadata.object_name = "x".repeat(240);
    assert_eq!(
        message.to_kvn().unwrap_err().code(),
        Some("validation.out_of_range")
    );
}

#[test]
fn reference_frame_epoch_is_calendar_form_in_xml() {
    let kvn = KVN_FIXTURES[0].replacen(
        "REF_FRAME = EME2000\n",
        "REF_FRAME = EME2000\nREF_FRAME_EPOCH = 2000-01-01T12:00:00\n",
        1,
    );
    let oem = Oem::from_kvn(&kvn).expect("calendar frame epoch should parse");
    let xml = oem.to_xml().expect("valid OEM should generate");
    assert!(xml.contains("<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>"));

    let numeric = xml.replace(
        "<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>",
        "<REF_FRAME_EPOCH>123.5</REF_FRAME_EPOCH>",
    );
    assert!(Oem::from_xml(&numeric).is_err());
}

#[test]
fn generation_rejects_mutated_contextual_epochs() {
    let mut oem = Oem::from_xml(XML).expect("fixture should parse");
    oem.body.segment[0].metadata.start_time = ccsds_ndm::types::Epoch::new("+").unwrap();
    assert!(oem.to_xml().is_err());
    assert!(oem.to_kvn().is_err());

    let mut oem = Oem::from_xml(XML).expect("fixture should parse");
    oem.body.segment[0].data.state_vector[0].epoch =
        ccsds_ndm::types::Epoch::new("2023-02-29T00:00:00").unwrap();
    assert!(oem.to_xml().is_err());
}
