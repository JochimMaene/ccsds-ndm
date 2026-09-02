# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

import inspect
from pathlib import Path

import pytest

import ccsds_ndm

REPOSITORY_ROOT = Path(__file__).parents[3]
OPM_KVN = (REPOSITORY_ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn").read_text()
OEM_KVN = (REPOSITORY_ROOT / "ccsds-ndm/data/kvn/oem_g11.kvn").read_text()
COMBINED_XML = (REPOSITORY_ROOT / "ccsds-ndm/data/xml/ndm_g12.xml").read_text()
PERMISSIVE_XML = (REPOSITORY_ROOT / "ccsds-ndm/data/xml/ndm_g22.xml").read_text()
STANDALONE_KVN_CASES = [
    (ccsds_ndm.Omm, "omm", "omm_g7.kvn", False),
    (ccsds_ndm.Ocm, "ocm", "ocm_g15.kvn", True),
    (ccsds_ndm.Cdm, "cdm", "cdm_362.kvn", False),
    (ccsds_ndm.Tdm, "tdm", "tdm_e1.kvn", True),
    (ccsds_ndm.Rdm, "rdm", "rdm_c1.kvn", False),
    (ccsds_ndm.Aem, "aem", "aem_g4.kvn", True),
    (ccsds_ndm.Apm, "apm", "apm_g1.kvn", False),
    (ccsds_ndm.Acm, "acm", "acm_g6.kvn", True),
]


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


def test_python_opm_validation_raises_one_aggregate_error():
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    header = message.header
    header.originator = ""
    message.header = header
    message.segment.metadata.object_name = ""

    with pytest.raises(ccsds_ndm.NdmValidationError) as aggregate:
        message.validate()
    error = str(aggregate.value).lower()
    assert "originator" in error
    assert "object_name" in error

    with pytest.raises(TypeError):
        message.validate(strict=False)


def test_unsupported_version_errors_expose_the_common_diagnostic_attributes():
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    message.version = "1.0"

    with pytest.raises(ccsds_ndm.NdmValidationError) as unsupported:
        message.to_str("xml")

    assert unsupported.value.operation == "generate"
    assert unsupported.value.code == "generation.unsupported_output_version"
    assert unsupported.value.line is None


def test_unsupported_file_format_has_no_side_effect(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    output = tmp_path / "output.ndm"

    with pytest.raises(ValueError, match="Unsupported format"):
        ccsds_ndm.to_file(message, str(output), "json")

    assert not output.exists()


def test_failed_generation_preserves_existing_file(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    output = tmp_path / "output.ndm"
    output.write_text("keep me")

    message.version = "1.0"
    with pytest.raises(
        ccsds_ndm.NdmValidationError, match="Unsupported KVN output version 1.0"
    ):
        ccsds_ndm.to_file(message, str(output), "kvn")

    assert output.read_text() == "keep me"


def test_opm_structured_diagnostics_and_parse_limits_are_exposed():
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

    with pytest.raises(ccsds_ndm.NdmError) as limited_parse:
        ccsds_ndm.Opm.from_str(
            OPM_KVN,
            format="kvn",
            max_input_bytes=len(OPM_KVN.encode()) - 1,
        )
    assert limited_parse.value.code == "resource.input_limit_exceeded"
    assert limited_parse.value.operation == "parse"


def test_python_opm_file_parsing_applies_limits_in_the_rust_core(tmp_path):
    source = tmp_path / "source.kvn"
    source.write_text(OPM_KVN)

    with pytest.raises(ccsds_ndm.NdmError) as limited:
        ccsds_ndm.from_file(str(source), format="kvn", max_input_bytes=16)
    assert limited.value.code == "resource.input_limit_exceeded"

    with pytest.raises(ccsds_ndm.NdmError) as auto_limited:
        ccsds_ndm.from_file(str(source), max_input_bytes=16)
    assert auto_limited.value.code == "resource.input_limit_exceeded"
    assert auto_limited.value.operation == "parse"
    assert auto_limited.value.notation is None
    assert auto_limited.value.message_kind is None

    with pytest.raises(ccsds_ndm.NdmError) as generic_limited:
        ccsds_ndm.from_file(str(source), max_input_bytes=16)
    assert generic_limited.value.code == "resource.input_limit_exceeded"
    assert generic_limited.value.operation == "parse"
    assert generic_limited.value.notation is None
    assert generic_limited.value.message_kind is None

    with pytest.raises(ccsds_ndm.NdmIoError):
        ccsds_ndm.from_file(str(tmp_path / "missing.kvn"), format="kvn")


def test_python_oem_uses_the_shared_parse_limits(tmp_path):
    with pytest.raises(ccsds_ndm.NdmError) as input_limit:
        ccsds_ndm.Oem.from_str(
            OEM_KVN,
            format="kvn",
            max_input_bytes=len(OEM_KVN.encode()) - 1,
        )
    assert input_limit.value.code == "resource.input_limit_exceeded"

    with pytest.raises(ccsds_ndm.NdmError) as record_limit:
        ccsds_ndm.Oem.from_str(OEM_KVN, format="kvn", max_records=0)
    assert record_limit.value.code == "resource.record_limit_exceeded"

    source = tmp_path / "source.oem"
    source.write_text(OEM_KVN)
    with pytest.raises(ccsds_ndm.NdmError) as file_limit:
        ccsds_ndm.from_file(
            str(source),
            format="kvn",
            max_input_bytes=16,
        )
    assert file_limit.value.code == "resource.input_limit_exceeded"


def test_parse_resource_limits_are_advanced_keyword_only_options():
    generic = inspect.signature(ccsds_ndm.from_str)
    assert generic.parameters["max_input_bytes"].kind is inspect.Parameter.KEYWORD_ONLY
    assert generic.parameters["max_records"].kind is inspect.Parameter.KEYWORD_ONLY

    oem = inspect.signature(ccsds_ndm.Oem.from_str)
    assert oem.parameters["max_input_bytes"].kind is inspect.Parameter.KEYWORD_ONLY
    assert oem.parameters["max_records"].kind is inspect.Parameter.KEYWORD_ONLY

    with pytest.raises(TypeError):
        ccsds_ndm.Oem.from_str(OEM_KVN, "kvn", 100, 100)
    with pytest.raises(TypeError, match="max_xml_depth"):
        ccsds_ndm.from_str(COMBINED_XML, format="xml", max_xml_depth=1)


@pytest.mark.parametrize("wrapper,kind,fixture,has_records", STANDALONE_KVN_CASES)
def test_remaining_python_messages_share_the_bounded_contract(
    wrapper, kind, fixture, has_records
):
    data = (REPOSITORY_ROOT / "ccsds-ndm/data/kvn" / fixture).read_text()

    with pytest.raises(ccsds_ndm.NdmError) as input_limit:
        wrapper.from_str(data, format="kvn", max_input_bytes=1)
    assert input_limit.value.code == "resource.input_limit_exceeded"
    assert input_limit.value.message_kind == kind

    if has_records:
        with pytest.raises(ccsds_ndm.NdmError) as record_limit:
            wrapper.from_str(data, format="kvn", max_records=0)
        assert record_limit.value.code == "resource.record_limit_exceeded"
        assert record_limit.value.message_kind == kind


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


def test_combined_python_message_keeps_identity_and_shared_limits():
    empty = ccsds_ndm.from_str("<ndm/>", format="xml")
    assert isinstance(empty, ccsds_ndm.Ndm)
    assert empty.messages == []

    message = ccsds_ndm.from_str(COMBINED_XML, format="xml")
    assert isinstance(message, ccsds_ndm.Ndm)

    with pytest.raises(ccsds_ndm.NdmError) as input_limit:
        ccsds_ndm.Ndm.from_str(COMBINED_XML, format="xml", max_input_bytes=1)
    assert input_limit.value.code == "resource.input_limit_exceeded"
    assert input_limit.value.message_kind == "ndm"

    with pytest.raises(ccsds_ndm.NdmError) as record_limit:
        ccsds_ndm.Ndm.from_str(COMBINED_XML, format="xml", max_records=0)
    assert record_limit.value.code == "resource.record_limit_exceeded"
    assert record_limit.value.message_kind == "ndm"

    xml = message.to_str("xml")
    assert isinstance(ccsds_ndm.from_str(xml, format="xml"), ccsds_ndm.Ndm)


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
