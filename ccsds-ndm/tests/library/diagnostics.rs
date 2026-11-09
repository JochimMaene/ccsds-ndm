use ccsds_ndm::error::DiagnosticOperation;
use ccsds_ndm::messages::{
    acm::Acm, aem::Aem, apm::Apm, cdm::Cdm, ndm::CombinedNdm, ocm::Ocm, oem::Oem, omm::Omm,
    opm::Opm, rdm::Rdm, tdm::Tdm,
};
use ccsds_ndm::{from_str, Ndm};

use crate::common::mutated_once;

#[test]
fn typed_and_detected_parsing_share_diagnostics() {
    fn check<T: Ndm + std::fmt::Debug>(kvn: &str, xml: &str) {
        for input in [
            format!("{}\nBOGUS = 1\n", kvn.lines().next().unwrap()),
            mutated_once(kvn, "ORIGINATOR =", "ORIGINATOR =\nBOGUS ="),
            mutated_once(kvn, "CREATION_DATE =", "CREATION_DATE = INVALID"),
        ] {
            let typed = T::from_kvn(&input).unwrap_err();
            let detected = from_str(&input).unwrap_err();
            let diagnostic = typed.diagnostic().expect("typed KVN parse context");
            assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
            assert_eq!(Some(diagnostic), detected.diagnostic(), "{input}");
        }
        for input in [
            mutated_once(xml, "<header>", "<header><BOGUS/>"),
            mutated_once(xml, "<ORIGINATOR>", "<ORIGINATOR/><ORIGINATOR>"),
            mutated_once(xml, "<CREATION_DATE>", "<CREATION_DATE>INVALID"),
        ] {
            let typed = T::from_xml(&input).unwrap_err();
            let detected = from_str(&input).unwrap_err();
            let diagnostic = typed.diagnostic().expect("typed XML parse context");
            assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
            assert_eq!(Some(diagnostic), detected.diagnostic(), "{input}");
        }
    }
    check::<Opm>(
        include_str!("../../data/kvn/opm_g2.kvn"),
        include_str!("../../data/xml/opm_g5.xml"),
    );
    check::<Omm>(
        include_str!("../../data/kvn/omm_g9.kvn"),
        include_str!("../../data/xml/omm_g10.xml"),
    );
    check::<Oem>(
        include_str!("../../data/kvn/oem_g13.kvn"),
        include_str!("../../data/xml/oem_g14.xml"),
    );
    check::<Ocm>(
        include_str!("../../data/kvn/ocm_g15.kvn"),
        include_str!("../../data/xml/ocm_g20.xml"),
    );
    let acm = include_str!("../../data/kvn/acm_g6.kvn");
    check::<Acm>(acm, &Acm::from_kvn(acm).unwrap().to_xml().unwrap());
    check::<Aem>(
        include_str!("../../data/kvn/aem_g4.kvn"),
        include_str!("../../data/xml/aem_g11.xml"),
    );
    check::<Apm>(
        include_str!("../../data/kvn/apm_g1.kvn"),
        include_str!("../../data/xml/apm_g10.xml"),
    );
    check::<Cdm>(
        include_str!("../../data/kvn/cdm_362.kvn"),
        include_str!("../../data/xml/cdm_44.xml"),
    );
    check::<Rdm>(
        include_str!("../../data/kvn/rdm_c1.kvn"),
        include_str!("../../data/xml/rdm_c4.xml"),
    );
    check::<Tdm>(
        include_str!("../../data/kvn/tdm_e1.kvn"),
        include_str!("../../data/xml/tdm_e21.xml"),
    );

    let input = mutated_once(
        include_str!("../../data/xml/ndm_g21.xml"),
        "<ndm",
        "<ndm BOGUS=\"1\"",
    );
    let typed = CombinedNdm::from_xml(&input).unwrap_err();
    let detected = from_str(&input).unwrap_err();
    assert!(typed.diagnostic().is_some());
    assert_eq!(typed.diagnostic(), detected.diagnostic());
}

#[test]
fn invalid_optional_cdm_enum_points_to_its_own_record() {
    let input = mutated_once(
        include_str!("../../data/kvn/cdm_362.kvn"),
        "REF_FRAME = EME2000",
        "REF_FRAME = INVALID",
    );
    let error = from_str(&input).unwrap_err();
    let diagnostic = error.diagnostic().unwrap();
    assert_eq!(diagnostic.code, Some("parse.invalid_enum"));
    assert_eq!(diagnostic.original_token, Some("INVALID"));
    assert_eq!(diagnostic.byte_offset, input.find("INVALID"));
    let line = input[..input.find("INVALID").unwrap()]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1;
    assert_eq!(
        diagnostic.source_location,
        Some((line, "REF_FRAME = ".len() + 1))
    );
    assert!(error.to_string().contains("REF_FRAME"), "{error}");
}

#[test]
fn xml_diagnostics_count_all_line_endings_consistently() {
    let input = mutated_once(
        include_str!("../../data/xml/oem_g14.xml"),
        "<X>",
        "<X>INVALID",
    );
    let lf_error = Oem::from_xml(&input).unwrap_err();
    let expected = lf_error.diagnostic().unwrap().source_location.unwrap();
    assert!(expected.0 > 1);
    for ending in ["\r\n", "\r"] {
        let input = input.replace('\n', ending);
        let error = Oem::from_xml(&input).unwrap_err();
        assert_eq!(error.diagnostic().unwrap().source_location, Some(expected));
    }
}
