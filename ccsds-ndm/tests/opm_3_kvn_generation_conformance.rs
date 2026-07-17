use ccsds_ndm::generation::VersionedNdm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::options::GenerateOptions;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::GmUnits;
use ccsds_ndm::MessageType;

fn opm() -> Opm {
    Opm::from_kvn(include_str!("../../data/kvn/opm_g1.kvn"))
        .expect("Annex G OPM fixture should parse")
}

fn assignment_keys(kvn: &str) -> Vec<&str> {
    kvn.lines()
        .filter(|line| !line.starts_with("COMMENT"))
        .filter_map(|line| line.split_once('=').map(|(key, _)| key.trim()))
        .collect()
}

fn assignment_units(kvn: &str) -> Vec<(&str, Option<&str>)> {
    kvn.lines()
        .filter(|line| !line.starts_with("COMMENT"))
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            let value = value.trim();
            let unit = value
                .strip_suffix(']')
                .and_then(|value| value.rsplit_once('['))
                .map(|(_, unit)| unit);
            Some((key.trim(), unit))
        })
        .collect()
}

fn comments(kvn: &str) -> Vec<&str> {
    kvn.lines()
        .filter_map(|line| line.strip_prefix("COMMENT "))
        .collect()
}

fn structural_lines(kvn: &str) -> Vec<String> {
    kvn.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            if line.starts_with("COMMENT ") {
                line.to_owned()
            } else {
                line.split_once('=')
                    .map_or_else(|| line.to_owned(), |(key, _)| key.trim().to_owned())
            }
        })
        .collect()
}

fn significant_digits(value: &str) -> usize {
    let digits: Vec<_> = value
        .bytes()
        .take_while(|byte| *byte != b'e' && *byte != b'E')
        .filter(u8::is_ascii_digit)
        .collect();
    let first = digits.iter().position(|digit| *digit != b'0');
    let last = digits.iter().rposition(|digit| *digit != b'0');
    match (first, last) {
        (Some(first), Some(last)) => last - first + 1,
        _ => 1,
    }
}

#[test]
fn annex_g_opm_fixtures_generate_printable_ordered_kvn_with_preserved_units() {
    for source in [
        include_str!("../../data/kvn/opm_g1.kvn"),
        include_str!("../../data/kvn/opm_g2.kvn"),
        include_str!("../../data/kvn/opm_g3.kvn"),
        include_str!("../../data/kvn/opm_g4.kvn"),
    ] {
        let generated = Opm::from_kvn(source)
            .expect("Annex G OPM fixture should parse")
            .to_kvn()
            .expect("Annex G OPM should generate");

        assert_eq!(assignment_keys(&generated), assignment_keys(source));
        assert_eq!(assignment_units(&generated), assignment_units(source));
        assert_eq!(comments(&generated), comments(source));
        assert_eq!(structural_lines(&generated), structural_lines(source));
        assert!(generated.lines().all(|line| {
            line.len() <= 254 && line.bytes().all(|byte| (b' '..=b'~').contains(&byte))
        }));
        for value in generated.lines().filter_map(|line| {
            let (_, value) = line.split_once('=')?;
            let number = value
                .trim()
                .split_once(" [")
                .map_or(value.trim(), |(number, _)| number);
            number.parse::<f64>().ok().map(|_| number)
        }) {
            assert!(
                value.contains('.') && significant_digits(value) <= 16,
                "non-compliant numeric spelling: {value}"
            );
        }
    }
}

#[test]
fn opm_kvn_preserves_exact_extreme_values_when_the_odm_can_represent_them() {
    let mut message = opm();
    message.body.segment.data.state_vector.x.value = f64::from_bits(1);

    let generated = message
        .to_kvn()
        .expect("minimum subnormal is representable");
    assert!(generated.contains("X                    = 5.0e-324"));
    let reparsed = Opm::from_kvn(&generated).expect("generated KVN should parse");
    assert_eq!(
        reparsed.body.segment.data.state_vector.x.value.to_bits(),
        f64::from_bits(1).to_bits()
    );
}

#[test]
fn opm_kvn_rejects_values_that_need_seventeen_digits_before_writing() {
    let mut message = opm();
    message.body.segment.data.state_vector.x.value = 1.234_567_890_123_456_7;

    let error = message
        .to_kvn()
        .expect_err("lossy 16-digit rounding must not occur");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.state_vector.x")
    );

    let mut output = Vec::new();
    message
        .write_kvn_to(&mut output, &GenerateOptions::source())
        .expect_err("streaming must run numeric preflight");
    assert!(output.is_empty());

    message
        .to_xml()
        .expect("the KVN-specific precision rule must not affect XML");
}

#[test]
fn opm_kvn_requires_the_odm_spelling_for_gm_units() {
    let mut message =
        Opm::from_kvn(include_str!("../../data/kvn/opm_g2.kvn")).expect("fixture should parse");
    message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .gm
        .units = Some(GmUnits::KM3PerS2);

    let error = message
        .to_kvn()
        .expect_err("XML-only uppercase GM units must not reach KVN");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.keplerian_elements.gm.units")
    );
}

#[test]
fn opm_kvn_is_identical_across_public_generation_entry_points() {
    let message =
        Opm::from_kvn(include_str!("../../data/kvn/opm_g4.kvn")).expect("fixture should parse");
    let expected = message.to_kvn().expect("typed generation should succeed");

    assert_eq!(
        message
            .to_kvn_with(&GenerateOptions::source())
            .expect("versioned generation should succeed"),
        expected
    );

    let mut streamed = Vec::new();
    message
        .write_kvn_to(&mut streamed, &GenerateOptions::source())
        .expect("streaming generation should succeed");
    assert_eq!(streamed, expected.as_bytes());

    let erased = MessageType::Opm(message);
    assert_eq!(
        erased
            .to_kvn()
            .expect("type-erased generation should succeed"),
        expected
    );
    assert_eq!(
        erased
            .to_kvn_with(&GenerateOptions::source())
            .expect("type-erased versioned generation should succeed"),
        expected
    );

    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let path = directory.path().join("opm.kvn");
    erased
        .to_kvn_file(&path)
        .expect("file generation should succeed");
    assert_eq!(
        std::fs::read(path).expect("generated file should be readable"),
        expected.as_bytes()
    );
}

#[test]
fn opm_kvn_rejects_non_ascii_and_control_characters() {
    for invalid in ["ESOC 🚀", "ESOC\tFlight Dynamics", "ESOC\nFlight Dynamics"] {
        let mut message = opm();
        message.header.originator = invalid.to_owned();

        let error = message
            .to_kvn()
            .expect_err("invalid KVN text must be rejected");
        assert_eq!(error.code(), Some("validation.invalid_value"));
        assert_eq!(error.field_path().as_deref(), Some("header.originator"));
    }
}

#[test]
fn opm_kvn_rejects_invalid_user_defined_keyword_suffixes() {
    let mut message =
        Opm::from_kvn(include_str!("../../data/kvn/opm_g4.kvn")).expect("fixture should parse");
    message
        .body
        .segment
        .data
        .user_defined_parameters
        .as_mut()
        .unwrap()
        .user_defined[0]
        .parameter = "earth model".to_owned();

    let error = message
        .to_kvn()
        .expect_err("KVN keywords must be uppercase and contain no blanks");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.user_defined_parameters.user_defined.parameter")
    );
}

#[test]
fn opm_kvn_enforces_the_254_character_line_limit() {
    let mut message = opm();
    message.header.comment = vec!["A".repeat(246)];

    let generated = message
        .to_kvn()
        .expect("a 254-character COMMENT line should be valid");
    assert_eq!(generated.lines().nth(1).unwrap().len(), 254);

    message.header.comment = vec!["A".repeat(247)];
    let error = message
        .to_kvn()
        .expect_err("a 255-character COMMENT line must be rejected");
    assert_eq!(error.code(), Some("validation.out_of_range"));
    assert_eq!(error.field_path().as_deref(), Some("header.comment"));
}

#[test]
fn invalid_opm_kvn_is_rejected_across_public_generation_entry_points() {
    let mut message = opm();
    message.header.originator = "ESOC 🚀".to_owned();

    assert!(message.to_kvn().is_err());
    assert!(message.to_kvn_with(&GenerateOptions::source()).is_err());

    let mut output = Vec::new();
    let error = message
        .write_kvn_to(&mut output, &GenerateOptions::source())
        .expect_err("invalid KVN text must be rejected");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(error.field_path().as_deref(), Some("header.originator"));
    assert!(output.is_empty());

    let erased = MessageType::Opm(message);
    assert!(erased.to_kvn().is_err());
    assert!(erased.to_kvn_with(&GenerateOptions::source()).is_err());

    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let path = directory.path().join("opm.kvn");
    std::fs::write(&path, b"unchanged").expect("sentinel should be written");
    assert!(erased.to_kvn_file(&path).is_err());
    assert_eq!(
        std::fs::read(path).expect("sentinel should remain readable"),
        b"unchanged"
    );
}
