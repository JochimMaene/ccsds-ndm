# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

from pathlib import Path

import pytest

import ccsds_ndm

REPOSITORY_ROOT = Path(__file__).parents[3]
OPM_KVN = (REPOSITORY_ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn").read_text()
# 1.0 is readable but withdrawn as an output edition, so it can only be reached by parsing.
OPM_KVN_V1 = OPM_KVN.replace("CCSDS_OPM_VERS = 3.0", "CCSDS_OPM_VERS = 1.0")
COMBINED_XML = (REPOSITORY_ROOT / "ccsds-ndm/data/xml/ndm_g12.xml").read_text()
PERMISSIVE_XML = (REPOSITORY_ROOT / "ccsds-ndm/data/xml/ndm_g22.xml").read_text()


def test_parsing_rejects_semantically_invalid_messages():
    with pytest.raises(ccsds_ndm.NdmValidationError):
        ccsds_ndm.from_str(PERMISSIVE_XML)


def test_generation_preserves_source_version():
    legacy = OPM_KVN.replace("3.0", "2.0", 1)
    message = ccsds_ndm.Opm.from_str(legacy, format="kvn")

    preserved = message.to_str("kvn")
    assert preserved.splitlines()[0].endswith("2.0")


def test_generation_has_no_misleading_unchecked_mode():
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")

    with pytest.raises(TypeError, match="validate"):
        message.to_str("kvn", validate=False)

    with pytest.raises(TypeError):
        message.validate(strict=False)


def test_unsupported_version_errors_expose_the_common_diagnostic_attributes():
    message = ccsds_ndm.Opm.from_str(OPM_KVN_V1, format="kvn")

    with pytest.raises(ccsds_ndm.NdmValidationError) as unsupported:
        message.to_str("xml")

    assert unsupported.value.operation == "generate"
    assert unsupported.value.code == "generation.unsupported_output_version"
    assert unsupported.value.line is None


def test_unsupported_file_format_has_no_side_effect(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    output = tmp_path / "output.ndm"

    with pytest.raises(ValueError, match="Unsupported format"):
        message.to_file(str(output), "json")

    assert not output.exists()


def test_format_names_are_case_insensitive(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="KVN")
    xml = message.to_str("XML")
    output = tmp_path / "output.xml"

    message.to_file(str(output), "XML")

    assert isinstance(ccsds_ndm.from_str(xml, format="XML"), ccsds_ndm.Opm)
    assert isinstance(ccsds_ndm.from_file(str(output), format="XML"), ccsds_ndm.Opm)
    assert ccsds_ndm.convert(OPM_KVN, "XML") == xml


def test_failed_generation_preserves_existing_file(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN_V1, format="kvn")
    output = tmp_path / "output.ndm"
    output.write_text("keep me")

    with pytest.raises(
        ccsds_ndm.NdmValidationError, match="Unsupported KVN output version 1.0"
    ):
        message.to_file(str(output), "kvn")

    assert output.read_text() == "keep me"


def test_opm_structured_diagnostics_are_exposed():
    invalid = OPM_KVN.replace(
        "OBJECT_NAME = OSPREY 5",
        "OBJECT_NAME = OSPREY 5\nUNKNOWN_KEY = value",
    )
    with pytest.raises(ccsds_ndm.NdmKvnParseError) as caught:
        ccsds_ndm.Opm.from_str(invalid, format="kvn")

    error = caught.value
    assert error.operation == "parse"
    assert error.notation == "kvn"
    assert error.message_kind == "opm"
    assert error.source_edition == "3.0"
    assert error.code == "parse.kvn.syntax"
    assert (error.line, error.column) == (6, 1)
    assert error.original_token == "UNKNOWN_KEY = value"


def test_python_file_parsing_reports_io_errors(tmp_path):
    source = tmp_path / "source.kvn"
    source.write_text(OPM_KVN)

    with pytest.raises(ccsds_ndm.NdmIoError):
        ccsds_ndm.from_file(str(tmp_path / "missing.kvn"), format="kvn")


def test_generic_python_conversion_dispatches_non_opm_messages(tmp_path):
    omm = (REPOSITORY_ROOT / "ccsds-ndm/data/kvn/omm_g7.kvn").read_text()
    xml = ccsds_ndm.convert(omm, "xml")
    assert isinstance(ccsds_ndm.from_str(xml, format="xml"), ccsds_ndm.Omm)

    source = tmp_path / "source.omm"
    destination = tmp_path / "destination.xml"
    source.write_text(omm)
    ccsds_ndm.convert_file(str(source), str(destination), "xml")
    assert isinstance(
        ccsds_ndm.from_file(str(destination), format="xml"), ccsds_ndm.Omm
    )


def test_combined_python_message_keeps_identity():
    empty = ccsds_ndm.from_str("<ndm/>", format="xml")
    assert isinstance(empty, ccsds_ndm.CombinedNdm)
    assert empty.messages == []

    message = ccsds_ndm.from_str(COMBINED_XML, format="xml")
    assert isinstance(message, ccsds_ndm.CombinedNdm)

    xml = message.to_str("xml")
    assert isinstance(ccsds_ndm.from_str(xml, format="xml"), ccsds_ndm.CombinedNdm)


def test_python_conversion_delegates_to_strict_rust_core(tmp_path):
    xml = ccsds_ndm.convert(OPM_KVN, "xml")
    expected_kvn = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn").to_str("kvn")
    assert ccsds_ndm.Opm.from_str(xml, format="xml").to_str("kvn") == expected_kvn

    kvn = ccsds_ndm.convert(xml, "kvn")
    assert ccsds_ndm.Opm.from_str(kvn, format="kvn").to_str("kvn") == expected_kvn

    source = tmp_path / "source.kvn"
    destination = tmp_path / "destination.xml"
    source.write_text(OPM_KVN)
    ccsds_ndm.convert_file(str(source), str(destination), "xml")
    ccsds_ndm.from_file(str(destination), format="xml")

    destination.write_text("sentinel")
    source.write_text("not an OPM")
    with pytest.raises(ccsds_ndm.NdmUnsupportedMessageError):
        ccsds_ndm.convert_file(str(source), str(destination), "xml")
    assert destination.read_text() == "sentinel"
