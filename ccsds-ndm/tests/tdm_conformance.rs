use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::tdm::{Tdm, TdmObservationData};
use ccsds_ndm::types::{Percentage, TdmRangeUnits};
use ccsds_ndm::{Ndm, Validate};
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/tdm_e1.kvn");
const XML: &str = include_str!("../data/xml/tdm_e21.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn tdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let time_system = "TIME_SYSTEM = UTC";
    let participant = "PARTICIPANT_1 = DSS-25";
    let observation = "TRANSMIT_FREQ_2 = 2005-159T17:41:00 32023442781.733";
    for (label, source) in [
        (
            "duplicate header keyword",
            KVN.replace("ORIGINATOR = NASA", "ORIGINATOR = NASA\nORIGINATOR = NASA"),
        ),
        (
            "duplicate metadata keyword",
            KVN.replace(time_system, &format!("{time_system}\n{time_system}")),
        ),
        (
            "unknown metadata keyword",
            KVN.replace(time_system, &format!("{time_system}\nUNKNOWN = value")),
        ),
        (
            "comment after an observation",
            KVN.replace(observation, &format!("{observation}\nCOMMENT misplaced")),
        ),
        ("unknown block", KVN.replace("META_START", "UNKNOWN_START")),
        (
            "mismatched block end",
            KVN.replace("META_STOP", "DATA_STOP"),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(participant, &format!("{participant} €")),
        ),
    ] {
        assert!(Tdm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn tdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let epoch = "<EPOCH>2007-069T15:22:22.000</EPOCH>";
    let observable = "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>";
    for (label, source) in [
        (
            "unknown metadata child",
            XML.replace("<metadata>", "<metadata><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown observation child",
            XML.replace("<observation>", "<observation><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown container attribute",
            XML.replace("<metadata>", "<metadata unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            XML.replace(
                observable,
                "<TRANSMIT_FREQ_1 unexpected=\"value\">7167941264.0</TRANSMIT_FREQ_1>",
            ),
        ),
        (
            "duplicate epoch",
            XML.replacen(epoch, &format!("{epoch}{epoch}"), 1),
        ),
        (
            "reordered observation members",
            XML.replacen(
                &format!("{epoch}\n{observable}"),
                &format!("{observable}\n{epoch}"),
                1,
            ),
        ),
    ] {
        assert!(Tdm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn tdm_xml_accepts_the_schema_defined_optional_observation_units() {
    let angle = XML.replacen(
        "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
        "<ANGLE_1 units=\"deg\">1.0</ANGLE_1>",
        1,
    );
    Tdm::from_xml(&angle).expect("ANGLE_1 units=deg is allowed by the TDM schema");

    let humidity = XML.replacen(
        "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
        "<RHUMIDITY units=\"%\">50.0</RHUMIDITY>",
        1,
    );
    let mut message =
        Tdm::from_xml(&humidity).expect("RHUMIDITY units=% is allowed by the TDM schema");
    assert!(
        message.to_kvn().is_err(),
        "non-ASCII XML text is not representable in KVN"
    );
    message.body.segments[0].metadata.participant_1 = "DSS-25".into();
    let normalized = Tdm::from_kvn(&message.to_kvn().unwrap()).unwrap();
    match &normalized.body.segments[0].data.observations[0].data {
        ccsds_ndm::messages::tdm::TdmObservationData::Rhumidity(value) => {
            assert_eq!(value.value, 50.0);
            assert!(value.units.is_none());
        }
        other => panic!("expected normalized humidity, got {other:?}"),
    }
}

#[test]
fn tdm_xml_accepts_every_schema_range_units_spelling() {
    for (units, canonical, spellings) in [
        (TdmRangeUnits::Km, "km", ["km", "KM"]),
        (TdmRangeUnits::Seconds, "s", ["s", "S"]),
        (TdmRangeUnits::Ru, "RU", ["RU", "ru"]),
    ] {
        let mut message = Tdm::from_kvn(include_str!("../data/kvn/tdm_e9.kvn")).unwrap();
        message.body.segments[0].metadata.range_units = Some(units);
        let xml = message.to_xml().unwrap();
        for spelling in spellings {
            let candidate = xml.replace(
                &format!("<RANGE_UNITS>{canonical}</RANGE_UNITS>"),
                &format!("<RANGE_UNITS>{spelling}</RANGE_UNITS>"),
            );
            Tdm::from_xml(&candidate).unwrap_or_else(|error| panic!("{spelling}: {error}"));
        }
    }
}

fn assert_tdm_rejects(message: &Tdm, field: &str) {
    let error = message
        .validate()
        .expect_err("invalid edited value accepted");
    assert!(error.to_string().contains(field), "{field}: {error}");
    assert!(message.to_kvn().is_err(), "KVN accepted invalid {field}");
    assert!(message.to_xml().is_err(), "XML accepted invalid {field}");

    let mut output = Vec::new();
    assert!(message.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty());
    assert!(message.write_xml_to(&mut output).is_err());
    assert!(output.is_empty());
}

#[test]
fn edited_tdm_metadata_numbers_are_revalidated_before_output() {
    for field in [
        "INTEGRATION_INTERVAL",
        "DOPPLER_COUNT_BIAS",
        "RANGE_MODULUS",
        "TRANSMIT_DELAY_1",
        "TRANSMIT_DELAY_2",
        "TRANSMIT_DELAY_3",
        "TRANSMIT_DELAY_4",
        "TRANSMIT_DELAY_5",
        "RECEIVE_DELAY_1",
        "RECEIVE_DELAY_2",
        "RECEIVE_DELAY_3",
        "RECEIVE_DELAY_4",
        "RECEIVE_DELAY_5",
        "FREQ_OFFSET",
        "CORRECTION_ANGLE_1",
        "CORRECTION_ANGLE_2",
        "CORRECTION_DOPPLER",
        "CORRECTION_MAG",
        "CORRECTION_RANGE",
        "CORRECTION_RCS",
        "CORRECTION_RECEIVE",
        "CORRECTION_TRANSMIT",
        "CORRECTION_ABERRATION_YEARLY",
        "CORRECTION_ABERRATION_DIURNAL",
    ] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        let metadata = &mut message.body.segments[0].metadata;
        let target = match field {
            "INTEGRATION_INTERVAL" => &mut metadata.integration_interval,
            "DOPPLER_COUNT_BIAS" => &mut metadata.doppler_count_bias,
            "RANGE_MODULUS" => &mut metadata.range_modulus,
            "TRANSMIT_DELAY_1" => &mut metadata.transmit_delay_1,
            "TRANSMIT_DELAY_2" => &mut metadata.transmit_delay_2,
            "TRANSMIT_DELAY_3" => &mut metadata.transmit_delay_3,
            "TRANSMIT_DELAY_4" => &mut metadata.transmit_delay_4,
            "TRANSMIT_DELAY_5" => &mut metadata.transmit_delay_5,
            "RECEIVE_DELAY_1" => &mut metadata.receive_delay_1,
            "RECEIVE_DELAY_2" => &mut metadata.receive_delay_2,
            "RECEIVE_DELAY_3" => &mut metadata.receive_delay_3,
            "RECEIVE_DELAY_4" => &mut metadata.receive_delay_4,
            "RECEIVE_DELAY_5" => &mut metadata.receive_delay_5,
            "FREQ_OFFSET" => &mut metadata.freq_offset,
            "CORRECTION_ANGLE_1" => &mut metadata.correction_angle_1,
            "CORRECTION_ANGLE_2" => &mut metadata.correction_angle_2,
            "CORRECTION_DOPPLER" => &mut metadata.correction_doppler,
            "CORRECTION_MAG" => &mut metadata.correction_mag,
            "CORRECTION_RANGE" => &mut metadata.correction_range,
            "CORRECTION_RCS" => &mut metadata.correction_rcs,
            "CORRECTION_RECEIVE" => &mut metadata.correction_receive,
            "CORRECTION_TRANSMIT" => &mut metadata.correction_transmit,
            "CORRECTION_ABERRATION_YEARLY" => &mut metadata.correction_aberration_yearly,
            _ => &mut metadata.correction_aberration_diurnal,
        };
        *target = Some(
            if field.starts_with("CORRECTION") || field == "FREQ_OFFSET" {
                f64::NAN
            } else if matches!(field, "INTEGRATION_INTERVAL" | "DOPPLER_COUNT_BIAS") {
                0.0
            } else {
                -1.0
            },
        );
        assert_tdm_rejects(&message, field);
    }

    for field in ["INTERPOLATION_DEGREE", "DOPPLER_COUNT_SCALE"] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        let metadata = &mut message.body.segments[0].metadata;
        if field == "INTERPOLATION_DEGREE" {
            metadata.interpolation_degree = Some(0);
        } else {
            metadata.doppler_count_scale = Some(0);
        }
        assert_tdm_rejects(&message, field);
    }
}

#[test]
fn tdm_paths_and_single_diff_receive_band_use_segment_context() {
    for path in ["2", "2,4"] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        message.body.segments[0].metadata.path.as_mut().unwrap().0 = path.into();
        assert_tdm_rejects(&message, "PATH");
    }

    let single_diff = include_str!("../data/kvn/tdm_e10.kvn");
    for data in [
        TdmObservationData::ReceiveFreq(1.0),
        TdmObservationData::Range(1.0),
    ] {
        let mut message = Tdm::from_kvn(single_diff).unwrap();
        message.body.segments[0].metadata.receive_band = None;
        message.body.segments[0].data.observations.truncate(1);
        message.body.segments[0].data.observations[0].data = data;
        assert_tdm_rejects(&message, "RECEIVE_BAND");
    }
    for data in [
        TdmObservationData::ReceiveFreq1(1.0),
        TdmObservationData::Dor(1.0),
        TdmObservationData::VlbiDelay(1.0),
    ] {
        let mut message = Tdm::from_kvn(single_diff).unwrap();
        message.body.segments[0].metadata.receive_band = None;
        message.body.segments[0].data.observations.truncate(1);
        message.body.segments[0].data.observations[0].data = data;
        message.validate().unwrap();
    }
}

#[test]
fn unambiguous_tdm_observation_domains_are_revalidated() {
    for (field, data) in [
        ("TRANSMIT_FREQ_1", TdmObservationData::TransmitFreq1(0.0)),
        ("TROPO_DRY", TdmObservationData::TropoDry(-1.0)),
        ("TROPO_WET", TdmObservationData::TropoWet(-1.0)),
        (
            "RHUMIDITY",
            TdmObservationData::Rhumidity(Percentage {
                value: 101.0,
                units: None,
            }),
        ),
    ] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        message.body.segments[0].data.observations.truncate(1);
        message.body.segments[0].data.observations[0].data = data;
        assert_tdm_rejects(&message, field);
    }
}

#[test]
fn tdm_observation_domains_follow_the_narrower_book_range() {
    // TDM 2.0 is narrower than the shared/TDM XSD for these observables; the book rule governs.
    for (field, data) in [
        ("ANGLE_1", TdmObservationData::Angle1(-180.000_000_1)),
        ("ANGLE_1", TdmObservationData::Angle1(360.0)),
        ("ANGLE_2", TdmObservationData::Angle2(-181.0)),
        ("ANGLE_2", TdmObservationData::Angle2(f64::INFINITY)),
        ("RCS", TdmObservationData::Rcs(0.0)),
        ("RCS", TdmObservationData::Rcs(-1.0)),
        ("STEC", TdmObservationData::Stec(0.0)),
        ("STEC", TdmObservationData::Stec(f64::NAN)),
        ("TEMPERATURE", TdmObservationData::Temperature(0.0)),
        ("TEMPERATURE", TdmObservationData::Temperature(-0.5)),
    ] {
        let mut message = Tdm::from_kvn(KVN).unwrap();
        message.body.segments[0].data.observations.truncate(1);
        message.body.segments[0].data.observations[0].data = data;
        assert_tdm_rejects(&message, field);
    }
}

#[test]
fn tdm_book_narrowed_boundaries_generate_valid_xml() {
    let mut message = Tdm::from_kvn(KVN).unwrap();
    let segment = &mut message.body.segments[0];
    let template = segment.data.observations[0].clone();
    segment.data.observations = [
        TdmObservationData::Angle1(-180.0),
        TdmObservationData::Angle2(359.999_999),
        TdmObservationData::Rcs(f64::MIN_POSITIVE),
        TdmObservationData::Stec(f64::MIN_POSITIVE),
        TdmObservationData::Temperature(f64::MIN_POSITIVE),
    ]
    .into_iter()
    .map(|data| {
        let mut observation = template.clone();
        observation.data = data;
        observation
    })
    .collect();

    message.validate().unwrap();
    let xml = message.to_xml().unwrap();
    validate_xml("TDM book-narrowed boundaries", &xml);
    assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
}

#[test]
fn tdm_revalidates_later_segments_and_observations() {
    let mut message = Tdm::from_kvn(KVN).unwrap();
    message.body.segments.push(message.body.segments[0].clone());
    message.body.segments[1].metadata.integration_interval = Some(0.0);
    assert_tdm_rejects(&message, "INTEGRATION_INTERVAL");

    let mut message = Tdm::from_kvn(KVN).unwrap();
    message.body.segments[0].data.observations[1].data = TdmObservationData::TransmitFreq1(0.0);
    assert_tdm_rejects(&message, "TRANSMIT_FREQ_1");
}

#[test]
fn unambiguous_tdm_numeric_boundaries_generate_valid_xml() {
    let mut message = Tdm::from_kvn(KVN).unwrap();
    let segment = &mut message.body.segments[0];
    segment.metadata.integration_interval = Some(f64::MIN_POSITIVE);
    segment.metadata.range_modulus = Some(0.0);
    segment.metadata.doppler_count_bias = Some(f64::MIN_POSITIVE);
    segment.metadata.doppler_count_scale = Some(1);
    segment.metadata.transmit_delay_1 = Some(0.0);
    segment.metadata.receive_delay_1 = Some(0.0);
    segment.metadata.interpolation = Some("LINEAR".into());
    segment.metadata.interpolation_degree = Some(1);

    let template = segment.data.observations[0].clone();
    segment.data.observations = [
        TdmObservationData::TransmitFreq1(f64::MIN_POSITIVE),
        TdmObservationData::TropoDry(0.0),
        TdmObservationData::TropoWet(0.0),
        TdmObservationData::Rhumidity(Percentage {
            value: 100.0,
            units: None,
        }),
    ]
    .into_iter()
    .map(|data| {
        let mut observation = template.clone();
        observation.data = data;
        observation
    })
    .collect();

    let xml = message.to_xml().unwrap();
    validate_xml("TDM numeric boundaries", &xml);
    assert_eq!(Tdm::from_xml(&xml).unwrap(), message);
}

#[test]
fn every_shipped_tdm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    let kvn_dir = repository_path("data/kvn");
    let mut kvn_files = fs::read_dir(&kvn_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("tdm_") && name.ends_with(".kvn"))
        })
        .collect::<Vec<_>>();
    kvn_files.sort();
    assert_eq!(kvn_files.len(), 21, "unexpected TDM KVN fixture inventory");

    for path in kvn_files {
        let name = path.file_name().unwrap().to_string_lossy();
        let source = fs::read_to_string(&path).unwrap();
        let message = Tdm::from_kvn(&source).unwrap_or_else(|error| panic!("{name}: {error}"));
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Tdm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Tdm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(&name, &xml);
    }

    for name in ["tdm_e21.xml", "tdm_e23.xml"] {
        let source = fs::read_to_string(repository_path(&format!("data/xml/{name}"))).unwrap();
        let message = Tdm::from_xml(&source).unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Tdm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }
}

fn validate_xml(label: &str, xml: &str) {
    let document = NamedTempFile::new().unwrap();
    fs::write(document.path(), xml).unwrap();
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(repository_path("data/xsd/ndmxml-4.0.0-master-4.0.xsd"))
        .arg(document.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{label} generated invalid XML: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
