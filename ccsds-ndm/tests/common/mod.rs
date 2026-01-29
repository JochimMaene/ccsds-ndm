// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // ccsds-ndm is in root/ccsds-ndm. data is in root/data.
    manifest_dir.parent().unwrap().join("data")
}
