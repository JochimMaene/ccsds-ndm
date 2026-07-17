// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, VersionedNdm};
use std::io::{self, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};

const OPM_3_KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");

#[derive(Debug)]
struct FailAfter {
    accepted: Vec<u8>,
    limit: usize,
}

impl FailAfter {
    fn new(limit: usize) -> Self {
        Self {
            accepted: Vec::new(),
            limit,
        }
    }
}

impl Write for FailAfter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let remaining = self.limit.saturating_sub(self.accepted.len());
        if remaining == 0 {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "deliberate test sink failure",
            ));
        }

        let accepted = remaining.min(buffer.len());
        self.accepted.extend_from_slice(&buffer[..accepted]);
        Ok(accepted)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn opm_3_xml_writer_propagates_sink_failures_without_panicking() {
    let opm = Opm::from_kvn(OPM_3_KVN).expect("failed to parse OPM 3.0 fixture");
    let expected = opm.to_xml().expect("failed to generate reference XML");
    let declaration_length = r#"<?xml version="1.0" encoding="UTF-8"?>"#.len();

    for limit in [
        0,
        declaration_length / 2,
        declaration_length + 1,
        expected.len() - 1,
    ] {
        let mut sink = FailAfter::new(limit);
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            opm.write_xml_to(&mut sink, &GenerateOptions::source())
        }));

        let result = outcome.unwrap_or_else(|_| panic!("writer panicked at byte limit {limit}"));
        let error = result.unwrap_err();
        assert_eq!(error.code(), Some("io.error"));
        assert_eq!(error.field_path(), None);
        let diagnostic = error
            .diagnostic()
            .expect("generation context should be present");
        assert_eq!(
            diagnostic.notation,
            ccsds_ndm::error::DiagnosticNotation::Xml
        );
        assert_eq!(
            diagnostic.message_kind,
            ccsds_ndm::validation::MessageKind::Opm
        );
        assert_eq!(diagnostic.field_path, None);
        let io_error = error.as_io_error().unwrap_or_else(|| {
            panic!("writer returned a non-I/O error at byte limit {limit}: {error}")
        });

        assert_eq!(io_error.kind(), io::ErrorKind::BrokenPipe);
        assert_eq!(io_error.to_string(), "deliberate test sink failure");
        assert_eq!(sink.accepted, expected.as_bytes()[..limit]);
    }
}
