use ccsds_ndm::error::Result;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Message;
use ccsds_ndm::Ndm;
use std::path::PathBuf;

#[test]
fn strict_public_parsing_signatures_remain_compatible() {
    let _typed_kvn: fn(&str) -> Result<Opm> = <Opm as Ndm>::from_kvn;
    let _typed_xml: fn(&str) -> Result<Opm> = <Opm as Ndm>::from_xml;
    let _detected_string: fn(&str) -> Result<Message> = ccsds_ndm::from_str;
    let _detected_file: fn(PathBuf) -> Result<Message> = ccsds_ndm::from_file::<PathBuf>;
}
