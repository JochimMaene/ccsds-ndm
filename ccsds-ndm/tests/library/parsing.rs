use crate::common::{mutated, mutated_once};
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::{from_str, Ndm};

#[test]
fn legacy_adm_and_tdm_editions_remain_parse_only() {
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
        let legacy_inputs = [
            mutated_once(input, "_VERS = 2.0", "_VERS = 1.0"),
            mutated_once(&xml, "version=\"2.0\"", "version=\"1.0\""),
        ];

        for legacy in legacy_inputs {
            let message = from_str(&legacy)
                .unwrap_or_else(|error| panic!("{message_type} 1.0 should parse: {error}"));
            assert_eq!(message.kind().as_str(), message_type);

            for error in [message.to_kvn().unwrap_err(), message.to_xml().unwrap_err()] {
                assert_eq!(
                    error.code(),
                    Some("generation.unsupported_output_version"),
                    "{message_type} returned the wrong error: {error}"
                );
            }
        }
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
            .user_defined[0]
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
            .user_defined[0]
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

    // OPM 1.0 is readable but was withdrawn as an output edition.
    assert!(supported_input_versions(MessageKind::Opm)
        .unwrap()
        .contains(&"1.0"));
    assert!(!supported_output_versions(MessageKind::Opm)
        .unwrap()
        .contains(&"1.0"));

    // The combined envelope carries no edition of its own.
    assert!(supported_input_versions(MessageKind::Ndm).is_none());
    assert!(supported_output_versions(MessageKind::Ndm).is_none());
}
