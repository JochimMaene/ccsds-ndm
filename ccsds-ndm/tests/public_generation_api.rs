use ccsds_ndm::error::Result;
use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::{Message, Ndm};
use std::path::PathBuf;

#[test]
fn validated_public_kvn_generation_signatures_remain_compatible() {
    let _typed: fn(&Opm) -> Result<String> = <Opm as Ndm>::to_kvn;
    let _typed_streaming: fn(&Opm, &mut Vec<u8>) -> Result<()> =
        <Opm as Ndm>::write_kvn_to::<Vec<u8>>;
    let _combined_streaming: fn(&CombinedNdm, &mut Vec<u8>) -> Result<()> =
        <CombinedNdm as Ndm>::write_xml_to::<Vec<u8>>;
    let _generic: fn(&Message) -> Result<String> = Message::to_kvn;
    let _generic_file: fn(&Message, PathBuf) -> Result<()> = Message::to_kvn_file::<PathBuf>;
}

/// Every family must reach the same bytes through the buffered and streaming entry points.
#[test]
fn streaming_generation_matches_buffered_output_for_every_family() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for fixture in [
        "kvn/acm_g6.kvn",
        "kvn/aem_g4.kvn",
        "kvn/apm_g1.kvn",
        "kvn/cdm_362.kvn",
        "kvn/ocm_g15.kvn",
        "kvn/oem_g11.kvn",
        "kvn/omm_g7.kvn",
        "kvn/opm_g1.kvn",
        "kvn/rdm_c1.kvn",
        "kvn/tdm_e1.kvn",
        "xml/ndm_g12.xml",
    ] {
        let message = ccsds_ndm::from_file(root.join("data").join(fixture)).unwrap();

        let mut xml = Vec::new();
        message.write_xml_to(&mut xml).unwrap();
        assert_eq!(String::from_utf8(xml).unwrap(), message.to_xml().unwrap());

        let mut kvn = Vec::new();
        let streamed = message.write_kvn_to(&mut kvn);
        match message {
            // Combined NDM has no KVN representation; both paths must refuse it.
            Message::Ndm(_) => {
                assert!(streamed.is_err(), "{fixture}");
                assert!(message.to_kvn().is_err(), "{fixture}");
                assert!(kvn.is_empty(), "{fixture}");
            }
            _ => {
                streamed.unwrap();
                assert_eq!(String::from_utf8(kvn).unwrap(), message.to_kvn().unwrap());
            }
        }
    }
}
