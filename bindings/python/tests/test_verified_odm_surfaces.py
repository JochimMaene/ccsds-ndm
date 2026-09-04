# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

from pathlib import Path

import pytest

import ccsds_ndm

DATA = Path(__file__).parents[3] / "ccsds-ndm/data"


@pytest.mark.parametrize(
    "wrapper,kind,kvn_name,xml_name",
    [
        (ccsds_ndm.Oem, "oem", "oem_g11.kvn", "oem_g14.xml"),
        (ccsds_ndm.Omm, "omm", "omm_g9.kvn", "omm_g10.xml"),
    ],
)
def test_verified_odm_python_surfaces_preserve_models_and_strictness(
    wrapper, kind, kvn_name, xml_name
):
    kvn = (DATA / "kvn" / kvn_name).read_text()
    shipped_xml = (DATA / "xml" / xml_name).read_text()

    expected = wrapper.from_str(kvn, format="kvn").to_str("kvn")
    assert isinstance(wrapper.from_str(shipped_xml, format="xml"), wrapper)

    xml = ccsds_ndm.convert(kvn, "xml")
    assert wrapper.from_str(xml, format="xml").to_str("kvn") == expected
    converted_kvn = ccsds_ndm.convert(xml, "kvn")
    assert wrapper.from_str(converted_kvn, format="kvn").to_str("kvn") == expected

    invalid_kvn = kvn.replace("OBJECT_NAME", "UNKNOWN_NAME", 1)
    with pytest.raises(ccsds_ndm.NdmFormatError) as invalid:
        wrapper.from_str(invalid_kvn, format="kvn")
    assert invalid.value.operation == "parse"
    assert invalid.value.notation == "kvn"
    assert invalid.value.message_kind == kind

    invalid_xml = xml.replace("<header>", "<header><UNKNOWN/>", 1)
    with pytest.raises(ccsds_ndm.NdmFormatError) as invalid:
        wrapper.from_str(invalid_xml, format="xml")
    assert invalid.value.operation == "parse"
    assert invalid.value.notation == "xml"
    assert invalid.value.message_kind == kind

    message = wrapper.from_str(kvn, format="kvn")
    message.header.originator = ""
    with pytest.raises(ccsds_ndm.NdmValidationError) as invalid:
        message.to_str("xml")
    assert invalid.value.operation == "generate"
    assert invalid.value.message_kind == kind

    with pytest.raises(ccsds_ndm.NdmError) as input_limited:
        wrapper.from_str(kvn, format="kvn", max_input_bytes=1)
    assert input_limited.value.code == "resource.input_limit_exceeded"
