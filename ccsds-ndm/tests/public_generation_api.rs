use ccsds_ndm::error::Result;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{MessageType, VersionedNdm};
use std::path::PathBuf;

#[test]
fn validated_public_kvn_generation_signatures_remain_compatible() {
    let _typed: fn(&Opm) -> Result<String> = <Opm as Ndm>::to_kvn;
    let _typed_streaming: fn(&Opm, &mut Vec<u8>) -> Result<()> =
        <Opm as VersionedNdm>::write_kvn_to::<Vec<u8>>;
    let _generic: fn(&MessageType) -> Result<String> = MessageType::to_kvn;
    let _generic_file: fn(&MessageType, PathBuf) -> Result<()> =
        MessageType::to_kvn_file::<PathBuf>;
}
