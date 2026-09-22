#[path = "../common/mod.rs"]
mod common;
mod epochs;
mod generation;
mod minimal;
mod model;
mod parsing;

const KVN: &str = include_str!("../../data/kvn/tdm_e1.kvn");
const XML: &str = include_str!("../../data/xml/tdm_e21.xml");
