use ccsds_ndm::error::{DiagnosticNotation, DiagnosticOperation};
use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::Ndm;

#[test]
fn combined_kvn_is_reported_as_an_unsupported_notation() {
    let error = CombinedNdm::from_kvn("CCSDS_OPM_VERS = 3.0\n").unwrap_err();
    let diagnostic = error.diagnostic().unwrap();
    assert_eq!(diagnostic.code, Some("unsupported.notation"));
    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Kvn);
    assert_eq!(
        diagnostic.message_kind,
        ccsds_ndm::validation::MessageKind::Ndm
    );

    let message = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: Vec::new(),
    };
    let mut output = Vec::new();
    for error in [
        message.to_kvn().unwrap_err(),
        ccsds_ndm::Message::Ndm(message)
            .write_kvn_to(&mut output)
            .unwrap_err(),
    ] {
        let diagnostic = error.diagnostic().unwrap();
        assert_eq!(diagnostic.code, Some("unsupported.notation"));
        assert_eq!(diagnostic.operation, DiagnosticOperation::Generate);
        assert_eq!(diagnostic.notation, DiagnosticNotation::Kvn);
        assert_eq!(diagnostic.source_edition, Some("combined"));
    }
    assert!(output.is_empty(), "a refused notation must emit zero bytes");
}
