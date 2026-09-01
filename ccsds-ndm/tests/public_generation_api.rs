use ccsds_ndm::error::Result;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, MessageType, VersionedNdm};
use std::path::PathBuf;

#[test]
fn validated_public_kvn_generation_signatures_remain_compatible() {
    let _typed: fn(&Opm) -> Result<String> = <Opm as Ndm>::to_kvn;
    let _typed_with: fn(&Opm, &GenerateOptions) -> Result<String> =
        <Opm as VersionedNdm>::to_kvn_with;
    let _typed_streaming: fn(&Opm, &mut Vec<u8>, &GenerateOptions) -> Result<()> =
        <Opm as VersionedNdm>::write_kvn_to::<Vec<u8>>;
    let _generic: fn(&MessageType) -> Result<String> = MessageType::to_kvn;
    let _generic_with: fn(&MessageType, &GenerateOptions) -> Result<String> =
        MessageType::to_kvn_with;
    let _generic_file: fn(&MessageType, PathBuf) -> Result<()> =
        MessageType::to_kvn_file::<PathBuf>;
}
