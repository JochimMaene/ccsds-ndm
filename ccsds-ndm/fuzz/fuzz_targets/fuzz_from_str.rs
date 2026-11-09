// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(message) = ccsds_ndm::from_str(s) {
            for output in [message.to_kvn(), message.to_xml()].into_iter().flatten() {
                assert!(ccsds_ndm::from_str(&output).is_ok());
            }
        }
    }
});
