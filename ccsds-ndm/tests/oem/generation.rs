use crate::common::{assert_rejects, mutated, mutated_once, validate_xml};
use crate::{KVN_FIXTURES, XML};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::{Message, Ndm};

#[test]
fn xml_degrees_outside_the_kvn_integer_range_fail_before_output() {
    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].metadata.interpolation_degree = Some("2147483648".parse().unwrap());
    let xml = message.to_xml().unwrap();
    validate_xml("positiveInteger degree", &xml);
    assert_eq!(Oem::from_xml(&xml).unwrap(), message);
    let mut output = Vec::new();
    assert_eq!(
        message.to_kvn().unwrap_err().code(),
        Some("validation.out_of_range")
    );
    assert!(message.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty());
}
#[test]
fn every_shipped_fixture_generates_deterministic_xsd_valid_xml_and_reparseable_kvn() {
    let messages = ["kvn", "xml"].into_iter().flat_map(|extension| {
        crate::common::fixtures("oem", extension)
            .into_iter()
            .map(move |(name, source)| {
                let message = if extension == "kvn" {
                    Oem::from_kvn(&source)
                } else {
                    Oem::from_xml(&source)
                }
                .unwrap_or_else(|error| panic!("{name}: {error}"));
                (name, message)
            })
    });

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
fn kvn_record_width_is_bounded_by_the_epoch_and_number_grammar() {
    // ODM 7.5.10 caps epoch fractions at the 16-digit fixed-point maximum and 7.5.7 caps
    // mantissas at 16 digits, so the widest valid record still fits a 254-character line.
    let mut message = Oem::from_xml(XML).unwrap();
    let state = &mut message.body.segment[0].data.state_vector[0];
    state.epoch = "2019-12-18T12:00:00.3311111111111111Z".parse().unwrap();
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
        *value = -1.797_693_134_862_315e308;
    }
    let kvn = message.to_kvn().unwrap();
    assert!(kvn.lines().all(|line| line.len() <= 254));

    message.body.segment[0].data.state_vector[0].epoch =
        "2019-12-18T12:00:00.33111111111111111".parse().unwrap();
    let mut output = Vec::new();
    let error = message
        .write_kvn_to(&mut output)
        .expect_err("a 17-digit epoch fraction must fail before writing");
    crate::common::assert_validation_field(&error, "stateVector EPOCH");
    assert!(output.is_empty());
}

#[test]
fn unsupported_editions_and_edition_specific_fields_are_rejected() {
    let mut historical = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    historical.version = "1.0".into();
    assert_eq!(
        historical.to_kvn().unwrap_err().code(),
        Some("generation.unsupported_output_version")
    );
    assert_eq!(
        historical.to_xml().unwrap_err().code(),
        Some("generation.unsupported_output_version")
    );

    let mut version_two = Oem::from_kvn(KVN_FIXTURES[0]).unwrap();
    version_two.version = "2.0".into();
    version_two.header.message_id = Some("OEM-3-ONLY".into());
    crate::common::assert_validation_field(&version_two.to_kvn().unwrap_err(), "MESSAGE_ID");
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
    let kvn = mutated_once(
        KVN_FIXTURES[0],
        "REF_FRAME = EME2000\n",
        "REF_FRAME = EME2000\nREF_FRAME_EPOCH = 2000-01-01T12:00:00\n",
    );
    let oem = Oem::from_kvn(&kvn).expect("calendar frame epoch should parse");
    let xml = oem.to_xml().expect("valid OEM should generate");
    assert!(xml.contains("<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>"));

    let numeric = mutated(
        &xml,
        "<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>",
        "<REF_FRAME_EPOCH>123.5</REF_FRAME_EPOCH>",
    );
    crate::common::assert_validation_field(
        &Oem::from_xml(&numeric).unwrap_err(),
        "REF_FRAME_EPOCH",
    );
}

#[test]
fn generation_rejects_mutated_contextual_epochs() {
    let mut oem = Oem::from_xml(XML).expect("fixture should parse");
    oem.body.segment[0].metadata.start_time = ccsds_ndm::types::Epoch::new("+").unwrap();
    assert_rejects(&oem, "START_TIME");

    let mut oem = Oem::from_xml(XML).expect("fixture should parse");
    oem.body.segment[0].data.state_vector[0].epoch =
        ccsds_ndm::types::Epoch::new("2023-02-29T00:00:00").unwrap();
    // KVN checks its absolute-time rule before the shared epoch validator.
    // Both paths must still identify the same invalid field.
    let mut output = Vec::new();
    for error in [
        oem.to_kvn().unwrap_err(),
        oem.to_xml().unwrap_err(),
        oem.write_kvn_to(&mut output).unwrap_err(),
        oem.write_xml_to(&mut output).unwrap_err(),
    ] {
        assert_eq!(error.code(), Some("validation.invalid_value"));
        assert_eq!(
            error.field_path().as_deref(),
            Some("body.segment[0].data.state_vector[0].epoch")
        );
    }
    assert!(output.is_empty());
}

#[test]
fn oem_generation_rejects_non_finite_state_vectors() {
    let mut oem = Oem::from_xml(XML).unwrap();
    oem.body.segment[0].data.state_vector[0].x.value = f64::NAN;

    let error = oem.to_kvn().unwrap_err();
    assert!(error.to_string().contains("representable CCSDS number"));

    let mut oem = Oem::from_xml(XML).unwrap();
    oem.body.segment[0].data.covariance_matrix[0].cx_x.value = f64::INFINITY;

    let error = oem.to_kvn().unwrap_err();
    assert!(error.to_string().contains("representable CCSDS number"));
}

#[test]
fn oem_generation_handles_maximum_width_records_without_panicking() {
    // Widen every component of one state vector to the longest spelling the CCSDS digit limit
    // allows, so the record sits just under the 254-character line limit.
    let widest = f64::from_bits(f64::MAX.to_bits() - 2);
    let oem = oem_with_state_vector_components(&format!("{widest:e}"));

    let output = oem
        .to_kvn()
        .expect("the widest representable values should generate");
    assert!(output.contains("1.797693134862315e308"));
    // The emitted document must read back, which is what makes the width limit meaningful.
    Oem::from_kvn(&output).expect("widest records round-trip");
}

#[test]
fn oem_generation_rejects_values_the_ccsds_digit_limit_cannot_represent() {
    // Rounding `f64::MAX` to 16 digits overflows to infinity, so it has no CCSDS spelling.
    let oem = oem_with_state_vector_components("1.7976931348623157e308");

    let error = oem
        .to_kvn()
        .expect_err("f64::MAX has no representable CCSDS spelling");
    assert!(
        error.to_string().contains("representable CCSDS number"),
        "unexpected diagnostic: {error}"
    );
}

fn oem_with_state_vector_components(replacement: &str) -> Oem {
    let mut input = XML.to_owned();
    for value in [
        "2789.6", "-280.0", "-1746.8", "4.73", "-2.50", "-1.04", "0.008", "0.001", "-0.159",
    ] {
        input = mutated_once(&input, &format!(">{value}<"), &format!(">{replacement}<"));
    }
    Oem::from_xml(&input).unwrap()
}

#[test]
fn serializes_multiple_covariance_matrices_in_one_block() {
    let oem = Oem::from_kvn(include_str!("../../data/kvn/oem_g13.kvn")).unwrap();
    assert_eq!(oem.body.segment[0].data.covariance_matrix.len(), 2);

    let output = oem.to_kvn().unwrap();
    assert_eq!(output.matches("COVARIANCE_START").count(), 1);
    assert_eq!(output.matches("COVARIANCE_STOP").count(), 1);
    assert_eq!(output.matches("EPOCH").count(), 2);
}
