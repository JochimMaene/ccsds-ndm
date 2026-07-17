# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

from pathlib import Path

import pytest

import ccsds_ndm


REPOSITORY_ROOT = Path(__file__).parents[3]
OPM_KVN = (REPOSITORY_ROOT / "data/kvn/opm_g1.kvn").read_text()
PERMISSIVE_XML = (REPOSITORY_ROOT / "data/xml/ndm_g22.xml").read_text()


def test_parsing_rejects_semantically_invalid_messages():
    with pytest.raises(ccsds_ndm.NdmValidationError):
        ccsds_ndm.from_str(PERMISSIVE_XML)


def test_generation_preserves_source_version_or_upgrades_explicitly():
    legacy = OPM_KVN.replace("3.0", "2.0", 1)
    message = ccsds_ndm.Opm.from_str(legacy, format="kvn")

    with pytest.raises(ccsds_ndm.NdmValidationError, match="output version 2.0"):
        message.to_kvn()

    upgraded = message.to_kvn(version="latest")
    assert upgraded.splitlines()[0].endswith("3.0")


def test_unchecked_generation_is_rejected():
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")

    with pytest.raises(ValueError, match="unchecked generation"):
        message.to_str("kvn", validate=False)


def test_python_opm_validation_exposes_strict_and_aggregate_core_results():
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    header = message.header
    header.originator = ""
    message.header = header

    with pytest.raises(ccsds_ndm.NdmValidationError) as strict:
        message.validate()
    assert strict.value.code == "validation.missing_required_field"
    assert strict.value.field_path == "header.originator"

    errors = message.validate(strict=False)
    assert errors is not None
    assert any("originator" in error.lower() for error in errors)


def test_unsupported_file_format_has_no_side_effect(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    output = tmp_path / "output.ndm"

    with pytest.raises(ValueError, match="Unsupported format"):
        message.to_file(str(output), "json")

    assert not output.exists()


def test_failed_generation_preserves_existing_file(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    output = tmp_path / "output.ndm"
    output.write_text("keep me")

    with pytest.raises(ccsds_ndm.NdmValidationError, match="output version 1.0"):
        message.to_file(str(output), "kvn", version="1.0")

    assert output.read_text() == "keep me"


def test_opm_structured_diagnostics_and_resource_limits_are_exposed():
    invalid = OPM_KVN.replace(
        "OBJECT_NAME = OSPREY 5",
        "OBJECT_NAME = OSPREY 5\nUNKNOWN_KEY = value",
    )
    with pytest.raises(ccsds_ndm.NdmKvnParseError) as caught:
        ccsds_ndm.Opm.from_str(invalid, format="kvn")

    error = caught.value
    assert error.severity == "error"
    assert error.operation == "parse"
    assert error.notation == "kvn"
    assert error.message_kind == "opm"
    assert error.source_edition == "3.0"
    assert error.target_edition is None
    assert error.code == "parse.kvn.syntax"
    assert (error.line, error.column) == (6, 1)
    assert error.original_token == "UNKNOWN_KEY = value"
    assert error.recovery is None

    with pytest.raises(ccsds_ndm.NdmError) as limited_parse:
        ccsds_ndm.Opm.from_str(
            OPM_KVN,
            format="kvn",
            max_input_bytes=len(OPM_KVN.encode()) - 1,
        )
    assert limited_parse.value.code == "resource.input_limit_exceeded"
    assert limited_parse.value.operation == "parse"

    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    with pytest.raises(ccsds_ndm.NdmError) as limited_output:
        message.to_xml(max_output_bytes=1)
    assert limited_output.value.code == "resource.output_limit_exceeded"
    assert limited_output.value.operation == "generate"
    assert limited_output.value.notation == "xml"


def test_python_opm_file_parsing_applies_limits_in_the_rust_core(tmp_path):
    source = tmp_path / "source.kvn"
    source.write_text(OPM_KVN)

    with pytest.raises(ccsds_ndm.NdmError) as limited:
        ccsds_ndm.Opm.from_file(str(source), format="kvn", max_input_bytes=16)
    assert limited.value.code == "resource.input_limit_exceeded"

    with pytest.raises(ccsds_ndm.NdmIoError):
        ccsds_ndm.Opm.from_file(str(tmp_path / "missing.kvn"), format="kvn")


def test_python_opm_conversion_delegates_to_strict_rust_core(tmp_path):
    xml = ccsds_ndm.convert_opm(OPM_KVN, "kvn", "xml")
    expected_kvn = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn").to_kvn()
    assert ccsds_ndm.Opm.from_str(xml, format="xml").to_kvn() == expected_kvn

    kvn = ccsds_ndm.convert_opm(xml, "xml", "kvn")
    assert ccsds_ndm.Opm.from_str(kvn, format="kvn").to_kvn() == expected_kvn

    source = tmp_path / "source.kvn"
    destination = tmp_path / "destination.xml"
    source.write_text(OPM_KVN)
    ccsds_ndm.convert_opm_file(str(source), str(destination), "kvn", "xml")
    ccsds_ndm.Opm.from_file(str(destination), format="xml")

    destination.write_text("sentinel")
    source.write_text("not an OPM")
    with pytest.raises(ccsds_ndm.NdmKvnParseError):
        ccsds_ndm.convert_opm_file(str(source), str(destination), "kvn", "xml")
    assert destination.read_text() == "sentinel"
