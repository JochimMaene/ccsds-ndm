use ccsds_ndm::messages::{
    acm::Acm, aem::Aem, apm::Apm, cdm::Cdm, ocm::Ocm, oem::Oem, omm::Omm, opm::Opm, rdm::Rdm,
    tdm::Tdm,
};
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{from_str_with_options, Message, Notation, ParseOptions};

#[test]
fn opm_preserves_comment_before_user_defined_after_optional_maneuvers() {
    let source = include_str!("../data/kvn/opm_g4.kvn").replace(
        "USER_DEFINED_EARTH_MODEL",
        "COMMENT belongs to user parameters\nUSER_DEFINED_EARTH_MODEL",
    );
    let message = Opm::from_kvn(&source).unwrap();
    assert_eq!(
        message
            .body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap()
            .comment,
        ["belongs to user parameters"]
    );
}

#[test]
fn opm_rejects_maneuver_fields_without_ignition_epoch() {
    let source = include_str!("../data/kvn/opm_g4.kvn").replace(
        "USER_DEFINED_EARTH_MODEL",
        "MAN_DURATION = 10.0 [s]\n\
         MAN_DELTA_MASS = -1.0 [kg]\n\
         MAN_REF_FRAME = TNW\n\
         MAN_DV_1 = 0.0 [km/s]\n\
         MAN_DV_2 = 0.0 [km/s]\n\
         MAN_DV_3 = 0.0 [km/s]\n\
         USER_DEFINED_EARTH_MODEL",
    );
    assert!(Opm::from_kvn(&source).is_err());
}

#[test]
fn oem_preserves_the_first_covariance_comment() {
    let source = include_str!("../data/kvn/oem_g13.kvn").replace(
        "COVARIANCE_START\n",
        "COVARIANCE_START\nCOMMENT belongs to first covariance\n",
    );
    let message = Oem::from_kvn(&source).unwrap();
    assert_eq!(
        message.body.segment[0].data.covariance_matrix[0].comment,
        ["belongs to first covariance"]
    );
}

#[test]
fn oem_enforces_integer_range_but_accepts_large_fixed_point_numbers() {
    let source = include_str!("../data/kvn/oem_g13.kvn");
    let integer = source.replacen("-2432.166", "3000000000", 1);
    let decimal = source.replacen("-2432.166", "3000000000.0", 1);
    Oem::from_kvn(&integer).expect_err("ODM integer-form values are limited to signed 32-bit");
    let decimal_message = Oem::from_kvn(&decimal).unwrap();
    assert_eq!(
        decimal_message.body.segment[0].data.state_vector[0].x.value,
        3_000_000_000.0
    );
}

#[test]
fn user_defined_values_may_contain_assignment_delimiters() {
    let omm_source = include_str!("../data/kvn/omm_g9.kvn").replace(
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
        include_str!("../data/kvn/rdm_c1.kvn")
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
fn oem_2_kvn_rejects_oem_3_header_fields() {
    let mut oem = Oem::from_kvn(include_str!("../data/kvn/oem_g11.kvn")).unwrap();
    oem.version = "2.0".into();
    oem.header.message_id = Some("OEM-3-ONLY".into());
    assert!(oem.to_kvn().is_err());
}

#[test]
fn assignment_values_may_end_with_marked_block_suffixes() {
    let apm = include_str!("../data/kvn/apm_g1.kvn")
        .replace("OBJECT_NAME = TRMM", "OBJECT_NAME = TRMM_START");
    Apm::from_kvn(&apm).unwrap();

    let aem = include_str!("../data/kvn/aem_g4.kvn").replacen(
        "OBJECT_NAME = MARS GLOBAL SURVEYOR",
        "OBJECT_NAME = MARS GLOBAL SURVEYOR_STOP",
        1,
    );
    Aem::from_kvn(&aem).unwrap();

    let acm = include_str!("../data/kvn/acm_g6.kvn").replace(
        "OBJECT_NAME = EUROBIRD-4A",
        "OBJECT_NAME = EUROBIRD-4A_START",
    );
    Acm::from_kvn(&acm).unwrap();

    let ocm = include_str!("../data/kvn/ocm_g15.kvn")
        .replace("CENTER_NAME = EARTH", "CENTER_NAME = EARTH_STOP");
    Ocm::from_kvn(&ocm).unwrap();
}

#[test]
fn strict_xml_rejects_forbidden_xml_1_characters() {
    let xml = include_str!("../data/xml/cdm_44.xml").replace("JSPOC", "JS\u{1}POC");
    assert!(Cdm::from_xml(&xml).is_err());
}

#[test]
fn xml_generation_rejects_forbidden_xml_1_characters_before_streaming() {
    let mut cdm = Cdm::from_kvn(include_str!("../data/kvn/cdm_362.kvn")).unwrap();
    cdm.header.originator = "JS\u{1}POC".into();
    assert!(cdm.to_xml().is_err());

    let mut output = Vec::new();
    assert!(cdm.write_xml_to(&mut output).is_err());
    assert!(output.is_empty());
}

#[test]
fn kvn_generation_rejects_non_ascii_and_control_text_before_streaming() {
    let mut apm = Apm::from_kvn(include_str!("../data/kvn/apm_g1.kvn")).unwrap();
    apm.header.originator = "GSFC-é".into();
    assert!(apm.to_kvn().is_err());

    let mut output = Vec::new();
    assert!(apm.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty());

    let mut tdm = Tdm::from_kvn(include_str!("../data/kvn/tdm_e1.kvn")).unwrap();
    tdm.header.originator = "NA\tSA".into();
    assert!(tdm.to_kvn().is_err());
}

#[test]
fn type_erased_file_errors_keep_non_opm_generation_context() {
    let message = Message::Oem(Oem::from_kvn(include_str!("../data/kvn/oem_g11.kvn")).unwrap());
    let directory = tempfile::tempdir().unwrap();

    for error in [
        message.to_kvn_file(directory.path()).unwrap_err(),
        message.to_xml_file(directory.path()).unwrap_err(),
    ] {
        let diagnostic = error.diagnostic().expect("file error should have context");
        assert_eq!(diagnostic.message_kind.as_str(), "OEM");
        assert_eq!(diagnostic.source_edition, Some("3.0"));
    }
}

#[test]
fn display_includes_structured_operation_context() {
    let parse_error = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\nBROKEN").unwrap_err();
    let parse_display = parse_error.to_string();
    assert!(parse_display.contains("failed to parse OPM KVN 3.0"));

    let mut message = Opm::from_kvn(include_str!("../data/kvn/opm_g1.kvn")).unwrap();
    message.body.segment.metadata.object_name.clear();
    let generation_display = message.to_xml().unwrap_err().to_string();
    assert!(generation_display.contains("failed to generate OPM XML 3.0"));
}

#[test]
fn multiline_xml_comments_convert_to_separate_kvn_records() {
    let xml = include_str!("../data/xml/opm_g5.xml")
        .replace("THIS IS AN XML VERSION OF THE OPM", "line one\nline two");
    let message = Opm::from_xml(&xml).unwrap();
    let kvn = message.to_kvn().unwrap();
    assert!(kvn.contains("COMMENT line one\nCOMMENT line two\n"));

    let mut streamed = Vec::new();
    message.write_kvn_to(&mut streamed).unwrap();
    assert_eq!(streamed, kvn.as_bytes());
}

#[test]
fn opm_maneuvers_are_not_history_records_in_either_notation() {
    let kvn = include_str!("../data/kvn/opm_g2.kvn");
    let options = ParseOptions::default().with_max_records(0);
    from_str_with_options(kvn, Some(Notation::Kvn), &options).unwrap();

    let xml = Opm::from_kvn(kvn).unwrap().to_xml().unwrap();
    from_str_with_options(&xml, Some(Notation::Xml), &options).unwrap();
}

#[test]
fn oem_comment_limit_matches_the_emitted_record() {
    let mut message = Oem::from_kvn(include_str!("../data/kvn/oem_g11.kvn")).unwrap();
    message.header.comment = vec!["x".repeat(235)];
    let kvn = message.to_kvn().unwrap();
    assert!(kvn.lines().any(|line| line.len() == 243));
}

#[test]
fn kvn_lexical_errors_have_the_same_category_for_string_and_streaming_output() {
    let mut message = Apm::from_kvn(include_str!("../data/kvn/apm_g1.kvn")).unwrap();
    message.header.originator = "GSFC-é".into();
    let direct = message.to_kvn().unwrap_err();
    let streaming = message.write_kvn_to(&mut Vec::new()).unwrap_err();
    assert!(direct.as_validation_error().is_some());
    assert!(streaming.as_validation_error().is_some());
}

#[test]
fn combined_xml_string_generation_identifies_invalid_envelope_fields() {
    let mut message =
        ccsds_ndm::messages::ndm::CombinedNdm::from_xml(include_str!("../data/xml/ndm_g12.xml"))
            .unwrap();
    message.id = Some("bad\u{1}id".into());
    let error = message.to_xml().unwrap_err();
    assert!(error.to_string().contains("MESSAGE_ID"));
}

#[test]
fn omm_rejects_kvn_numbers_it_could_not_spell_back() {
    // The schema types these fields as plain doubles, so finiteness and range checks let through
    // magnitudes whose shortest round-tripping spelling blows past the 254-character line limit.
    let mut message = Omm::from_kvn(include_str!("../data/kvn/omm_g7.kvn")).unwrap();
    message.body.segment.data.mean_elements.eccentricity.value = f64::MAX;

    let error = message.to_kvn().unwrap_err().to_string();
    assert!(
        error.contains("representable CCSDS number"),
        "unexpected error: {error}"
    );
}

#[test]
fn omm_kvn_numbers_use_odm_spellings_in_every_block() {
    let mut message = Omm::from_kvn(include_str!("../data/kvn/omm_g7.kvn")).unwrap();
    let data = &mut message.body.segment.data;
    data.mean_elements.eccentricity.value = 0.1 + 0.2;
    if let Some(mean_motion) = data.mean_elements.mean_motion.as_mut() {
        mean_motion.value = 0.1 + 0.2;
    }
    let tle = data.tle_parameters.as_mut().unwrap();
    tle.mean_motion_dot.value = 0.1 + 0.2;

    let kvn = message.to_kvn().unwrap();
    assert!(
        !kvn.contains("0.30000000000000004"),
        "Display spelling leaked into generated KVN:\n{kvn}"
    );
    for key in ["ECCENTRICITY", "MEAN_MOTION ", "MEAN_MOTION_DOT"] {
        let line = kvn
            .lines()
            .find(|line| line.starts_with(key))
            .unwrap_or_else(|| panic!("missing {key}"));
        assert!(
            !line.contains("0000000000000"),
            "unexpected {key} line: {line}"
        );
        assert!(line.len() <= 254, "{key} line exceeds 254 characters");
    }
}

#[test]
fn opm_xml_allows_a_comment_to_reopen_the_user_defined_group() {
    // `userDefinedType` wraps COMMENT and USER_DEFINED in an unbounded xsd:sequence, so the pair
    // may repeat and a COMMENT may legally follow a USER_DEFINED.
    let source = include_str!("../data/xml/opm_g5.xml").replace(
        "</data>",
        "<userDefinedParameters>\
         <COMMENT>first</COMMENT>\
         <USER_DEFINED parameter=\"A\">1</USER_DEFINED>\
         <COMMENT>second</COMMENT>\
         <USER_DEFINED parameter=\"B\">2</USER_DEFINED>\
         </userDefinedParameters></data>",
    );
    let message = Opm::from_xml(&source).unwrap();
    let user_defined = message
        .body
        .segment
        .data
        .user_defined_parameters
        .as_ref()
        .unwrap();
    assert_eq!(user_defined.comment, ["first", "second"]);
    assert_eq!(user_defined.user_defined.len(), 2);
}

#[test]
fn omm_xml_allows_a_comment_to_reopen_the_user_defined_group() {
    // The OMM carries the same `userDefinedType` block as the OPM and the RDM, so it has to
    // accept the repeating group on the same terms.
    let source = include_str!("../data/xml/omm_g10.xml").replace(
        "</data>",
        "<userDefinedParameters>\
         <COMMENT>first</COMMENT>\
         <USER_DEFINED parameter=\"A\">1</USER_DEFINED>\
         <COMMENT>second</COMMENT>\
         <USER_DEFINED parameter=\"B\">2</USER_DEFINED>\
         </userDefinedParameters></data>",
    );
    let message = Omm::from_xml(&source).unwrap();
    let user_defined = message
        .body
        .segment
        .data
        .user_defined_parameters
        .as_ref()
        .unwrap();
    assert_eq!(user_defined.comment, ["first", "second"]);
    assert_eq!(user_defined.user_defined.len(), 2);
}

#[test]
fn omm_xml_still_rejects_a_genuinely_out_of_order_child() {
    // The relaxation is scoped to the repeating group: `metadata` is a plain xsd:sequence, so
    // CENTER_NAME cannot precede OBJECT_ID.
    let source = include_str!("../data/xml/omm_g10.xml");
    let reordered = source.replace(
        "<OBJECT_ID>1995-025A</OBJECT_ID>\n<CENTER_NAME>EARTH</CENTER_NAME>",
        "<CENTER_NAME>EARTH</CENTER_NAME>\n<OBJECT_ID>1995-025A</OBJECT_ID>",
    );
    assert_ne!(source, reordered, "fixture shape changed; update this test");
    assert!(Omm::from_xml(&reordered).is_err());
}

#[test]
fn opm_xml_still_rejects_a_genuinely_out_of_order_child() {
    // Relaxing the rank check for repeating groups must not relax a plain sequence: `header` is
    // an ordinary xsd:sequence, so ORIGINATOR cannot precede CREATION_DATE.
    let source = include_str!("../data/xml/opm_g5.xml").replace(
        "<CREATION_DATE>",
        "<ORIGINATOR>early</ORIGINATOR><CREATION_DATE>",
    );
    let error = Opm::from_xml(&source).unwrap_err().to_string();
    assert!(
        error.contains("out-of-order child 'CREATION_DATE'"),
        "unexpected error: {error}"
    );
}

#[test]
fn self_closing_elements_count_towards_the_xml_depth_limit() {
    // A self-closing element occupies a nesting level even though it never opens a frame, so it
    // has to be measured against the limit the same way a start tag is. Here COMMENT is the only
    // thing at depth 3, and it is spelled empty.
    let document = concat!(
        r#"<opm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" "#,
        r#"id="CCSDS_OPM_VERS" version="3.0">"#,
        "<header><COMMENT/></header></opm>",
    );
    let options = ParseOptions::default().with_max_xml_depth(2);
    let error = ccsds_ndm::from_str_with_options(document, None, &options)
        .unwrap_err()
        .to_string();
    assert!(error.contains("xml_depth"), "unexpected error: {error}");
}

#[test]
fn nilled_elements_may_carry_their_units_attribute() {
    // XSD lets a nillable element keep its attributes, and `attribute_allowed` explicitly permits
    // `units` on MASS, so the envelope validator and serde have to agree that this parses.
    let source = include_str!("../data/xml/opm_g5.xml");
    let mass = source
        .lines()
        .find(|line| line.contains("<MASS"))
        .expect("fixture carries MASS");
    let nilled = source.replace(mass.trim(), "<MASS units=\"kg\" xsi:nil=\"true\"/>");
    let message = Opm::from_xml(&nilled).unwrap();
    let parameters = message
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .expect("fixture carries a spacecraft parameters block");
    assert!(parameters.mass.is_none());
    assert!(
        parameters.drag_coeff.is_some(),
        "the rest of the block still parses"
    );
}
