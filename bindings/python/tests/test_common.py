# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for the shared header classes (OdmHeader, AdmHeader).
"""

import pytest

from ccsds_ndm import AdmHeader, OdmHeader


@pytest.mark.parametrize("header_type", [OdmHeader, AdmHeader])
def test_creation_date_requires_absolute_epoch(header_type):
    with pytest.raises(ValueError):
        header_type("123.5", "TEST")

    header = header_type("2002-204T15:56:23Z", "TEST")
    assert header.creation_date == "2002-204T15:56:23Z"
    with pytest.raises(ValueError):
        header.creation_date = "123.5"
