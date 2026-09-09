# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""Regression tests for the shared binding-maintenance tooling."""

from audit_bindings import ccsds_reference
from binding_source import parse_rust_file

STRUCTS = """\
/// A message.
///
/// **CCSDS Reference**: 508.0-B-1, Section 1.
pub struct Message {
    pub undocumented_first: String,
    /// The second field.
    pub documented_second: String,
}

/// Another message.
pub struct Other {
    /// The first field.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 2.
    pub documented_first: String,
}
"""


def test_struct_docstring_does_not_leak_onto_an_undocumented_first_field(tmp_path):
    path = tmp_path / "sample.rs"
    path.write_text(STRUCTS)
    structs = parse_rust_file(path)

    message = structs["Message"]
    assert "CCSDS Reference" in message.docstring
    assert message.fields["undocumented_first"].docstring == ""
    assert message.fields["documented_second"].docstring == "The second field."

    # A documented first field must still be read correctly.
    other = structs["Other"]
    assert "Section 2." in other.fields["documented_first"].docstring


def test_ccsds_reference_ignores_harmless_formatting_but_not_the_citation():
    core = "**CCSDS Reference**:  508.0-B-1,  Section 3.2."
    python = "CCSDS Reference: 508.0-B-1, Section 3.2"
    assert ccsds_reference(core) == ccsds_reference(python) == "508.0-B-1, Section 3.2"

    assert ccsds_reference("CCSDS Reference: 999.9-Z-9, Section 42.") != ccsds_reference(core)
    assert ccsds_reference("no citation here") is None
    assert ccsds_reference(None) is None
