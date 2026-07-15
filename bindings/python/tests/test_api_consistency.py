# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""API consistency tests for class-level parse helpers."""

from pathlib import Path

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
