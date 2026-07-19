# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""API consistency tests for class-level parse helpers."""

from pathlib import Path

import ccsds_ndm
import pytest
from ccsds_ndm import Acm, Aem, Apm, Cdm, Ndm, Ocm, Oem, Omm, Opm, Rdm, Tdm


ROOT = Path(__file__).resolve().parents[3]

CLASS_FIXTURES = [
    (Acm, ROOT / "data/kvn/acm_g6.kvn"),
    (Aem, ROOT / "data/kvn/aem_g4.kvn"),
    (Apm, ROOT / "data/kvn/apm_g1.kvn"),
    (Cdm, ROOT / "data/kvn/cdm_362.kvn"),
    (Ndm, ROOT / "data/xml/ndm_g12.xml"),
    (Ocm, ROOT / "data/kvn/ocm_g15.kvn"),
    (Oem, ROOT / "data/kvn/oem_g11.kvn"),
    (Omm, ROOT / "data/kvn/omm_g7.kvn"),
    (Opm, ROOT / "data/kvn/opm_g1.kvn"),
    (Rdm, ROOT / "data/kvn/rdm_c1.kvn"),
    (Tdm, ROOT / "data/kvn/tdm_e1.kvn"),
]


@pytest.mark.parametrize(("cls", "path"), CLASS_FIXTURES)
def test_type_specific_from_file_allows_default_format(cls, path):
    parsed = cls.from_file(str(path))
    assert isinstance(parsed, cls)


@pytest.mark.parametrize(("cls", "path"), CLASS_FIXTURES)
def test_type_specific_from_str_allows_default_format(cls, path):
    parsed = cls.from_str(path.read_text())
    assert isinstance(parsed, cls)


def test_all_public_ndm_exceptions_share_the_documented_base():
    exception_types = [
        ccsds_ndm.NdmFormatError,
        ccsds_ndm.NdmKvnParseError,
        ccsds_ndm.NdmXmlError,
        ccsds_ndm.NdmValidationError,
        ccsds_ndm.NdmEpochError,
        ccsds_ndm.NdmIoError,
        ccsds_ndm.NdmUnsupportedMessageError,
    ]
    assert all(
        issubclass(exception, ccsds_ndm.NdmError) for exception in exception_types
    )


def test_edit_propagates_nested_and_list_changes_to_the_message():
    opm = Opm.from_file(str(ROOT / "data/kvn/opm_g1.kvn"))
    editable = ccsds_ndm.edit(opm)
    editable.segment.metadata.object_name = "UPDATED"
    editable.segment.data.state_vector.x = 7000.0
    assert opm.segment.metadata.object_name == "UPDATED"
    assert opm.segment.data.state_vector.x == pytest.approx(7000.0)
    kvn = opm.to_kvn()
    assert any(
        line.startswith("OBJECT_NAME") and line.endswith("= UPDATED")
        for line in kvn.splitlines()
    )
    assert any(
        line.startswith("X ") and line.endswith("= 7000.0") for line in kvn.splitlines()
    )

    cdm = Cdm.from_file(str(ROOT / "data/kvn/cdm_362.kvn"))
    ccsds_ndm.edit(cdm).body.segments[0].metadata.object_name = "OBJECT-1"
    assert cdm.body.segments[0].metadata.object_name == "OBJECT-1"
    assert any(
        line.startswith("OBJECT_NAME") and line.endswith("= OBJECT-1")
        for line in cdm.to_kvn().splitlines()
    )
