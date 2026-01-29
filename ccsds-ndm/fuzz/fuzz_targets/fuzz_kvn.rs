// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

#![no_main]

use libfuzzer_sys::fuzz_target;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::messages::{
    acm::Acm, aem::Aem, apm::Apm, cdm::Cdm, ocm::Ocm, oem::Oem, omm::Omm, opm::Opm, rdm::Rdm,
    tdm::Tdm,
};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Try all message types - none should panic
        let _ = Opm::from_kvn(s);
        let _ = Omm::from_kvn(s);
        let _ = Oem::from_kvn(s);
        let _ = Ocm::from_kvn(s);
        let _ = Tdm::from_kvn(s);
        let _ = Rdm::from_kvn(s);
        let _ = Cdm::from_kvn(s);
        let _ = Aem::from_kvn(s);
        let _ = Apm::from_kvn(s);
        let _ = Acm::from_kvn(s);
    }
});
