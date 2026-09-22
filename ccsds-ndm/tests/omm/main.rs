#[path = "../common/mod.rs"]
mod common;
mod conversion;
mod epochs;
mod generation;
mod minimal;
mod model;
mod parsing;

const KVN: &str = include_str!("../../data/kvn/omm_g9.kvn");
const XML: &str = include_str!("../../data/xml/omm_g10.xml");
// The only shipped OMM KVN fixture with a covariance matrix.
const KVN_WITH_COVARIANCE: &str = include_str!("../../data/kvn/omm_g8.kvn");
