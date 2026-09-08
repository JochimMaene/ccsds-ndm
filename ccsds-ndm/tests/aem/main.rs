#[path = "../common/mod.rs"]
mod common;
mod conversion;
mod generation;
mod strict_parsing;
mod validation;

pub(crate) const KVN: &str = include_str!("../../data/kvn/aem_g4.kvn");
pub(crate) const SPIN_KVN: &str = include_str!("../../data/kvn/aem_g5.kvn");
pub(crate) const XML: &str = include_str!("../../data/xml/aem_g13.xml");
