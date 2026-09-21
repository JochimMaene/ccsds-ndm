// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::error::CcsdsNdmError;
use ccsds_ndm::{from_file, from_str};
use std::fs;

#[test]
fn from_file_preserves_kvn_and_xml_models() {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("opm.kvn", include_str!("../data/kvn/opm_g1.kvn")),
        ("opm.xml", include_str!("../data/xml/opm_g5.xml")),
    ] {
        let path = directory.path().join(name);
        fs::write(&path, source).unwrap();
        assert_eq!(
            from_file(&path).unwrap(),
            from_str(source).unwrap(),
            "{name}"
        );
    }
}

#[test]
fn from_file_reports_missing_files() {
    let directory = tempfile::tempdir().unwrap();
    let error = from_file(directory.path().join("missing.kvn")).unwrap_err();
    assert!(
        matches!(
            error,
            CcsdsNdmError::Io(ref source) if source.kind() == std::io::ErrorKind::NotFound
        ),
        "expected a missing-file error, got {error}"
    );
}
