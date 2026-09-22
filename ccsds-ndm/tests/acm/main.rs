#[path = "../common/mod.rs"]
mod common;
mod epochs;
mod generation;
mod minimal;
mod parsing;

const ATT_KVN: &str = include_str!("../../data/kvn/acm_g7.kvn");
const COV_KVN: &str = include_str!("../../data/kvn/acm_g9.kvn");
