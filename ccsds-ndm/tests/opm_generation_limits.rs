use ccsds_ndm::error::DiagnosticNotation;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, VersionedNdm};

fn opm() -> Opm {
    Opm::from_kvn(include_str!("../data/kvn/opm_g4.kvn")).expect("rich OPM fixture should parse")
}

fn assert_limit_error(
    error: ccsds_ndm::error::CcsdsNdmError,
    notation: DiagnosticNotation,
    limit: usize,
    actual: usize,
) {
    assert_eq!(error.code(), Some("resource.output_limit_exceeded"));
    assert_eq!(error.field_path(), None);
    let diagnostic = error
        .diagnostic()
        .expect("resource failure should retain generation context");
    assert_eq!(diagnostic.notation, notation);
    assert_eq!(diagnostic.code, Some("resource.output_limit_exceeded"));
    assert_eq!(diagnostic.field_path, None);
    match error {
        ccsds_ndm::error::CcsdsNdmError::Generation { source, .. } => assert!(matches!(
            *source,
            ccsds_ndm::error::CcsdsNdmError::ResourceLimitExceeded {
                resource: "generated_document",
                limit: found_limit,
                actual: found_actual,
            } if found_limit == limit && found_actual == actual
        )),
        other => panic!("resource error omitted its generation wrapper: {other}"),
    }
}

#[test]
fn xml_total_output_limit_is_exact_and_preflighted_for_streaming() {
    let message = opm();
    let expected = message.to_xml().expect("unlimited generation should work");
    let exact = GenerateOptions::source().with_max_output_bytes(expected.len());
    let too_small = GenerateOptions::source().with_max_output_bytes(expected.len() - 1);

    assert_eq!(
        message
            .to_xml_with(&exact)
            .expect("exact materialized limit should work"),
        expected
    );
    assert_limit_error(
        message
            .to_xml_with(&too_small)
            .expect_err("small materialized limit should fail"),
        DiagnosticNotation::Xml,
        expected.len() - 1,
        expected.len(),
    );

    let mut output = Vec::new();
    assert_limit_error(
        message
            .write_xml_to(&mut output, &too_small)
            .expect_err("small streaming limit should fail"),
        DiagnosticNotation::Xml,
        expected.len() - 1,
        expected.len(),
    );
    assert!(output.is_empty(), "preflight failure emitted XML bytes");

    message
        .write_xml_to(&mut output, &exact)
        .expect("exact streaming limit should work");
    assert_eq!(output, expected.as_bytes());
}

#[test]
fn kvn_total_output_limit_is_exact_and_preflighted_for_streaming() {
    let message = opm();
    let expected = message.to_kvn().expect("unlimited generation should work");
    let exact = GenerateOptions::source().with_max_output_bytes(expected.len());
    let too_small = GenerateOptions::source().with_max_output_bytes(expected.len() - 1);

    assert_eq!(
        message
            .to_kvn_with(&exact)
            .expect("exact materialized limit should work"),
        expected
    );
    assert_limit_error(
        message
            .to_kvn_with(&too_small)
            .expect_err("small materialized limit should fail"),
        DiagnosticNotation::Kvn,
        expected.len() - 1,
        expected.len(),
    );

    let mut output = Vec::new();
    assert_limit_error(
        message
            .write_kvn_to(&mut output, &too_small)
            .expect_err("small streaming limit should fail"),
        DiagnosticNotation::Kvn,
        expected.len() - 1,
        expected.len(),
    );
    assert!(output.is_empty(), "preflight failure emitted KVN bytes");

    message
        .write_kvn_to(&mut output, &exact)
        .expect("exact streaming limit should work");
    assert_eq!(output, expected.as_bytes());
}
