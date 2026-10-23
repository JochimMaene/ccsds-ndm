use crate::common::{mutated, mutated_once};
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::{from_str, Ndm};

#[test]
fn unaudited_adm_and_tdm_editions_are_not_advertised_as_supported() {
    let cases = [
        ("AEM", include_str!("../../data/kvn/aem_g4.kvn")),
        ("APM", include_str!("../../data/kvn/apm_g1.kvn")),
        ("ACM", include_str!("../../data/kvn/acm_g6.kvn")),
        ("TDM", include_str!("../../data/kvn/tdm_e1.kvn")),
    ];

    for (message_type, input) in cases {
        let xml = from_str(input)
            .expect("2.0 fixture should parse")
            .to_xml()
            .expect("2.0 fixture should generate XML");
        // The edition is checked before the structure, so content the later edition lacks
        // reports the unsupported edition, not an unknown keyword or element.
        let legacy_inputs = [
            mutated_once(input, "_VERS = 2.0", "_VERS = 1.0\nLEGACY_KEYWORD = 1"),
            mutated_once(
                &mutated_once(&xml, "version=\"2.0\"", "version=\"1.0\""),
                "<header>",
                "<header><LEGACY/>",
            ),
        ];

        for legacy in legacy_inputs {
            let error = from_str(&legacy).unwrap_err();
            assert!(error.to_string().contains("1.0"), "{message_type}: {error}");
            assert_eq!(
                error.code(),
                Some("parse.unsupported_input_version"),
                "{message_type}: {error}"
            );
        }
        let kind = from_str(input).unwrap().kind();
        assert_eq!(
            ccsds_ndm::versioning::supported_input_versions(kind),
            Some(&["2.0"][..])
        );
    }
}

#[test]
fn user_defined_values_may_contain_assignment_delimiters() {
    let omm_source = mutated(
        include_str!("../../data/kvn/omm_g9.kvn"),
        "USER_DEFINED_EARTH_MODEL = WGS-84",
        "USER_DEFINED_EARTH_MODEL = a=b",
    );
    let omm = Omm::from_kvn(&omm_source).unwrap();
    assert_eq!(
        omm.body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap()
            .parameters()
            .next()
            .unwrap()
            .value,
        "a=b"
    );

    let rdm_source = format!(
        "{}\nUSER_DEFINED_EQUATION = a=b\n",
        include_str!("../../data/kvn/rdm_c1.kvn")
    );
    let rdm = Rdm::from_kvn(&rdm_source).unwrap();
    assert_eq!(
        rdm.body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap()
            .parameters()
            .next()
            .unwrap()
            .value,
        "a=b"
    );
}

#[test]
fn assignment_values_may_end_with_marked_block_suffixes() {
    let apm = mutated(
        include_str!("../../data/kvn/apm_g1.kvn"),
        "OBJECT_NAME = TRMM",
        "OBJECT_NAME = TRMM_START",
    );
    Apm::from_kvn(&apm).unwrap();

    // Both segments are renamed: the AEM now requires one object across the whole message.
    let aem = mutated(
        &mutated(
            include_str!("../../data/kvn/aem_g4.kvn"),
            "OBJECT_NAME = MARS GLOBAL SURVEYOR",
            "OBJECT_NAME = MARS GLOBAL SURVEYOR_STOP",
        ),
        "OBJECT_NAME = mars global surveyor",
        "OBJECT_NAME = mars global surveyor_STOP",
    );
    Aem::from_kvn(&aem).unwrap();

    let acm = mutated(
        include_str!("../../data/kvn/acm_g6.kvn"),
        "OBJECT_NAME = EUROBIRD-4A",
        "OBJECT_NAME = EUROBIRD-4A_START",
    );
    Acm::from_kvn(&acm).unwrap();

    let ocm = mutated(
        include_str!("../../data/kvn/ocm_g15.kvn"),
        "CENTER_NAME = EARTH",
        "CENTER_NAME = EARTH_STOP",
    );
    Ocm::from_kvn(&ocm).unwrap();
}

#[test]
fn supported_editions_separate_input_from_output() {
    use ccsds_ndm::validation::MessageKind;
    use ccsds_ndm::versioning::{supported_input_versions, supported_output_versions};

    for kind in [
        MessageKind::Opm,
        MessageKind::Omm,
        MessageKind::Oem,
        MessageKind::Ocm,
        MessageKind::Aem,
        MessageKind::Apm,
        MessageKind::Acm,
        MessageKind::Cdm,
        MessageKind::Tdm,
        MessageKind::Rdm,
    ] {
        let input = supported_input_versions(kind).expect("standalone family has editions");
        let output = supported_output_versions(kind).expect("standalone family has editions");
        assert!(!output.is_empty(), "{kind:?}");
        assert!(
            output.iter().all(|version| input.contains(version)),
            "{kind:?}: {output:?} not a subset of {input:?}"
        );
    }

    // OPM 1.0 is not advertised without an edition-specific implementation.
    assert!(!supported_input_versions(MessageKind::Opm)
        .unwrap()
        .contains(&"1.0"));
    assert!(!supported_output_versions(MessageKind::Opm)
        .unwrap()
        .contains(&"1.0"));

    // The combined envelope carries no edition of its own.
    assert!(supported_input_versions(MessageKind::Ndm).is_none());
    assert!(supported_output_versions(MessageKind::Ndm).is_none());
}

const OMM_KVN: &str = include_str!("../../data/kvn/omm_g7.kvn");
const OPM_KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");
const TDM_KVN: &str = include_str!("../../data/kvn/tdm_e18.kvn");

#[test]
fn kvn_integers_accept_a_leading_plus_sign() {
    // Every book allows a sign on integers (ODM 7.5.4); unsigned fields still refuse `-`.
    let signed = mutated(OMM_KVN, "NORAD_CAT_ID = 23581", "NORAD_CAT_ID = +23581");
    assert_eq!(from_str(&signed).unwrap(), from_str(OMM_KVN).unwrap());
    let signed = mutated(
        TDM_KVN,
        "INTERPOLATION_DEGREE = 7",
        "INTERPOLATION_DEGREE = +7",
    );
    assert_eq!(from_str(&signed).unwrap(), from_str(TDM_KVN).unwrap());
    let negative = mutated(OMM_KVN, "NORAD_CAT_ID = 23581", "NORAD_CAT_ID = -23581");
    assert!(from_str(&negative).is_err());
}

#[test]
fn trailing_junk_after_a_value_is_reported_at_its_keyword() {
    for (from, to, key) in [
        ("X = 6503.514000", "X = 6503.514000x", "X"),
        ("MASS = 3000.000000", "MASS = 1D-3", "MASS"),
        (
            "SOLAR_RAD_AREA = 18.770000",
            "SOLAR_RAD_AREA = 18.77x70 [m**2]",
            "SOLAR_RAD_AREA",
        ),
    ] {
        let line = OPM_KVN.lines().position(|line| line == from).unwrap() + 1;
        let error = from_str(&mutated(OPM_KVN, from, to)).unwrap_err();
        let kvn = crate::common::kvn_parse_error(&error)
            .unwrap_or_else(|| panic!("{to}: expected a KVN parse error, got {error}"));
        assert_eq!(kvn.line, line, "{to}");
        assert_eq!(kvn.message, format!("Invalid float for {key}"), "{to}");
    }
    let error = from_str(&mutated(
        OMM_KVN,
        "NORAD_CAT_ID = 23581",
        "NORAD_CAT_ID = 5.0",
    ))
    .unwrap_err();
    assert_eq!(
        crate::common::kvn_parse_error(&error).unwrap().message,
        "Invalid unsigned integer for NORAD_CAT_ID"
    );
}

#[test]
fn blank_or_na_optional_kvn_values_are_absent() {
    // One shared rule for every optional KVN value: blank or case-insensitive N/A is absence.
    let without = mutated(OPM_KVN, "MASS = 3000.000000\n", "");
    let without = from_str(&without).unwrap();
    for value in ["", "N/A", "n/a"] {
        let kvn = mutated(OPM_KVN, "MASS = 3000.000000", &format!("MASS = {value}"));
        assert_eq!(from_str(&kvn).unwrap(), without, "{value:?}");
    }
    let message_id = OMM_KVN
        .lines()
        .find(|line| line.starts_with("MESSAGE_ID"))
        .unwrap();
    let blank = from_str(&mutated(OMM_KVN, message_id, "MESSAGE_ID = N/A")).unwrap();
    assert_eq!(
        blank,
        from_str(&mutated(OMM_KVN, &format!("{message_id}\n"), "")).unwrap()
    );
}
