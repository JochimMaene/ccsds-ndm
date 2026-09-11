#[path = "../common/mod.rs"]
mod common;
mod conversion;
mod diagnostics;
mod generation;
mod strict_parsing;
mod validation;

pub(crate) const KVN_FIXTURES: [&str; 3] = [
    include_str!("../../data/kvn/oem_g11.kvn"),
    include_str!("../../data/kvn/oem_g12.kvn"),
    include_str!("../../data/kvn/oem_g13.kvn"),
];
pub(crate) const XML: &str = include_str!("../../data/xml/oem_g14.xml");
