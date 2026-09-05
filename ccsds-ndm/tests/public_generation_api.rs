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
