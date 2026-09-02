use ccsds_ndm::{from_str, from_str_with_options, MessageType, Notation, ParseOptions};

fn standalone_cases() -> [(&'static str, &'static str); 10] {
    [
        ("OPM", include_str!("../data/kvn/opm_g1.kvn")),
        ("OMM", include_str!("../data/kvn/omm_g7.kvn")),
        ("OEM", include_str!("../data/kvn/oem_g11.kvn")),
        ("OCM", include_str!("../data/kvn/ocm_g15.kvn")),
        ("CDM", include_str!("../data/kvn/cdm_362.kvn")),
        ("TDM", include_str!("../data/kvn/tdm_e1.kvn")),
        ("RDM", include_str!("../data/kvn/rdm_c1.kvn")),
        ("AEM", include_str!("../data/kvn/aem_g4.kvn")),
        ("APM", include_str!("../data/kvn/apm_g1.kvn")),
        ("ACM", include_str!("../data/kvn/acm_g6.kvn")),
    ]
}

#[test]
fn every_standalone_message_uses_the_shared_bounded_contract() {
    for (name, input) in standalone_cases() {
        let message = from_str_with_options(
            input,
            Some(Notation::Kvn),
            &ParseOptions::default().with_max_input_bytes(input.len()),
        )
        .unwrap_or_else(|error| panic!("{name} bounded parse failed: {error}"));
        let kind = message.kind();
        assert_eq!(kind.as_str(), name);

        let too_small_input = ParseOptions::default().with_max_input_bytes(input.len() - 1);
        let error =
            from_str_with_options(input, Some(Notation::Kvn), &too_small_input).unwrap_err();
        let diagnostic = error
            .diagnostic()
            .unwrap_or_else(|| panic!("{name} input limit lacked a diagnostic"));
        assert_eq!(diagnostic.message_kind, kind);
        assert_eq!(diagnostic.code, Some("resource.input_limit_exceeded"));

        for (notation, output) in [
            (Notation::Kvn, message.to_kvn().unwrap()),
            (Notation::Xml, message.to_xml().unwrap()),
        ] {
            let reparsed = from_str_with_options(&output, Some(notation), &ParseOptions::default())
                .unwrap_or_else(|error| panic!("{name} generated output did not parse: {error}"));
            assert_eq!(reparsed.kind(), kind);
        }
    }
}

#[test]
fn legacy_adm_and_tdm_editions_remain_parse_only() {
    let cases = [
        ("AEM", include_str!("../data/kvn/aem_g4.kvn")),
        ("APM", include_str!("../data/kvn/apm_g1.kvn")),
        ("ACM", include_str!("../data/kvn/acm_g6.kvn")),
        ("TDM", include_str!("../data/kvn/tdm_e1.kvn")),
    ];

    for (message_type, input) in cases {
        let xml = from_str(input)
            .expect("2.0 fixture should parse")
            .to_xml()
            .expect("2.0 fixture should generate XML");
        let legacy_inputs = [
            input.replacen("_VERS = 2.0", "_VERS = 1.0", 1),
            xml.replacen("version=\"2.0\"", "version=\"1.0\"", 1),
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
fn combined_ndm_keeps_its_identity() {
    let input = include_str!("../data/xml/ndm_g12.xml");
    let message = from_str(input).expect("combined NDM fixture should parse");
    assert!(matches!(message, MessageType::Ndm(_)));

    let output = message.to_xml().expect("combined NDM should generate");
    assert!(matches!(from_str(&output).unwrap(), MessageType::Ndm(_)));
}
