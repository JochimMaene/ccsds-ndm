# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

from pathlib import Path

import pytest

import ccsds_ndm


REPOSITORY_ROOT = Path(__file__).parents[3]
OPM_KVN = (REPOSITORY_ROOT / "data/kvn/opm_g1.kvn").read_text()
OEM_KVN = (REPOSITORY_ROOT / "data/kvn/oem_g11.kvn").read_text()
OEM_XML = (REPOSITORY_ROOT / "data/xml/oem_g14.xml").read_text()
COMBINED_XML = (REPOSITORY_ROOT / "data/xml/ndm_g12.xml").read_text()
PERMISSIVE_XML = (REPOSITORY_ROOT / "data/xml/ndm_g22.xml").read_text()
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
    assert strict.value.severity == "error"
    assert strict.value.operation == "validate"
    assert strict.value.line is None

    errors = message.validate(strict=False)
    assert errors is not None
    assert any("originator" in error.lower() for error in errors)


def test_unsupported_version_errors_expose_the_common_diagnostic_attributes():
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    message.version = "1.0"

    with pytest.raises(ccsds_ndm.NdmValidationError) as unsupported:
        message.to_str("xml")

    assert unsupported.value.severity == "error"
    assert unsupported.value.operation == "generate"
    assert unsupported.value.code == "generation.unsupported_output_version"
    assert unsupported.value.line is None


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


def test_python_oem_uses_the_shared_parse_and_generation_limits(tmp_path):
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

    with pytest.raises(ccsds_ndm.NdmError) as depth_limit:
        ccsds_ndm.Oem.from_str(OEM_XML, format="xml", max_xml_depth=1)
    assert depth_limit.value.code == "resource.xml_depth_limit_exceeded"

    message = ccsds_ndm.Oem.from_str(OEM_KVN, format="kvn")
    with pytest.raises(ccsds_ndm.NdmError) as output_limit:
        message.to_xml(max_output_bytes=1)
    assert output_limit.value.code == "resource.output_limit_exceeded"
    assert output_limit.value.message_kind == "oem"

    source = tmp_path / "source.oem"
    source.write_text(OEM_KVN)
    with pytest.raises(ccsds_ndm.NdmError) as file_limit:
        ccsds_ndm.Oem.from_file(
            str(source),
            format="kvn",
            max_input_bytes=16,
        )
    assert file_limit.value.code == "resource.input_limit_exceeded"


@pytest.mark.parametrize("wrapper,kind,fixture,has_records", STANDALONE_KVN_CASES)
def test_remaining_python_messages_share_the_bounded_contract(
    wrapper, kind, fixture, has_records
):
    data = (REPOSITORY_ROOT / "data/kvn" / fixture).read_text()

    with pytest.raises(ccsds_ndm.NdmError) as input_limit:
        wrapper.from_str(data, format="kvn", max_input_bytes=1)
    assert input_limit.value.code == "resource.input_limit_exceeded"
    assert input_limit.value.message_kind == kind

    if has_records:
        with pytest.raises(ccsds_ndm.NdmError) as record_limit:
            wrapper.from_str(data, format="kvn", max_records=0)
        assert record_limit.value.code == "resource.record_limit_exceeded"
        assert record_limit.value.message_kind == kind

    message = wrapper.from_str(data, format="kvn")
    with pytest.raises(ccsds_ndm.NdmError) as output_limit:
        message.to_xml(max_output_bytes=1)
    assert output_limit.value.code == "resource.output_limit_exceeded"
    assert output_limit.value.message_kind == kind


def test_generic_python_conversion_dispatches_non_opm_messages(tmp_path):
    omm = (REPOSITORY_ROOT / "data/kvn/omm_g7.kvn").read_text()
    xml = ccsds_ndm.convert(omm, "kvn", "xml", max_output_bytes=100_000)
    assert isinstance(ccsds_ndm.from_str(xml, format="xml"), ccsds_ndm.Omm)

    source = tmp_path / "source.omm"
    destination = tmp_path / "destination.xml"
    source.write_text(omm)
    ccsds_ndm.convert_file(str(source), str(destination), "kvn", "xml")
    assert isinstance(
        ccsds_ndm.from_file(str(destination), format="xml"), ccsds_ndm.Omm
    )


def test_combined_python_message_keeps_identity_and_shared_limits():
    message = ccsds_ndm.from_str(COMBINED_XML, format="xml")
    assert isinstance(message, ccsds_ndm.Ndm)

    with pytest.raises(ccsds_ndm.NdmError) as input_limit:
        ccsds_ndm.Ndm.from_str(COMBINED_XML, format="xml", max_input_bytes=1)
    assert input_limit.value.code == "resource.input_limit_exceeded"
    assert input_limit.value.message_kind == "ndm"

    with pytest.raises(ccsds_ndm.NdmError) as depth_limit:
        ccsds_ndm.Ndm.from_str(COMBINED_XML, format="xml", max_xml_depth=1)
    assert depth_limit.value.code == "resource.xml_depth_limit_exceeded"
    assert depth_limit.value.message_kind == "ndm"

    with pytest.raises(ccsds_ndm.NdmError) as record_limit:
        ccsds_ndm.Ndm.from_str(COMBINED_XML, format="xml", max_records=0)
    assert record_limit.value.code == "resource.record_limit_exceeded"
    assert record_limit.value.message_kind == "ndm"

    xml = message.to_xml()
    assert isinstance(ccsds_ndm.from_str(xml, format="xml"), ccsds_ndm.Ndm)
    with pytest.raises(ccsds_ndm.NdmError) as output_limit:
        message.to_xml(max_output_bytes=len(xml.encode()) - 1)
    assert output_limit.value.code == "resource.output_limit_exceeded"
    assert output_limit.value.message_kind == "ndm"


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
