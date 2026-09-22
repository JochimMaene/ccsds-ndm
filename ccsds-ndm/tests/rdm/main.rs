#[path = "../common/mod.rs"]
mod common;
mod epochs;
mod generation;
mod parsing;

const KVN: &str = include_str!("../../data/kvn/rdm_c2.kvn");
const XML: &str = include_str!("../../data/xml/rdm_c4.xml");
