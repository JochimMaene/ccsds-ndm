#[path = "../common/mod.rs"]
mod common;
mod epochs;
mod generation;
mod minimal;
mod model;
mod parsing;

const KVN: &str = include_str!("../../data/kvn/ocm_g18.kvn");
const XML: &str = include_str!("../../data/xml/ocm_g20.xml");
