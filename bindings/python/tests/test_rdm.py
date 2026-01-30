# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Reentry Data Message (RDM) Python bindings.
"""

import pytest
from ccsds_ndm import (
    AtmosphericReentryParameters,
    Rdm,
    RdmData,
    RdmHeader,
    RdmMetadata,
    RdmSegment,
)


class TestRdm:
    """Tests for RDM bindings."""

    def _create_valid_rdm(self):
        header = RdmHeader(
            originator="TEST", creation_date="2023-01-01T00:00:00", message_id="ID"
        )

        meta = RdmMetadata(
            object_name="SAT1",
            international_designator="2023-001A",
            epoch_tzero="2023-01-01T00:00:00",
            center_name="EARTH",
            time_system="UTC",
        )

        # AtmosphericReentryParameters signature:
        # new(*, orbit_lifetime, reentry_altitude, ...)
        atm_params = AtmosphericReentryParameters(
            orbit_lifetime=5.0, reentry_altitude=120.0
        )

        # RdmData signature:
        # new(*, atmospheric_reentry_parameters, ...)
        data = RdmData(atmospheric_reentry_parameters=atm_params)

        # RdmSegment signature:
        # new(*, metadata, data)
        seg = RdmSegment(metadata=meta, data=data)

        # Rdm signature:
        # new(*, header, segment)
        return Rdm(header=header, segment=seg)

    def test_roundtrip_kvn(self):
        try:
            rdm = self._create_valid_rdm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        kvn = rdm.to_str(format="kvn")
        assert "CCSDS_RDM_VERS" in kvn

        rdm2 = Rdm.from_str(kvn, format="kvn")
        assert rdm2.header.originator == "TEST"
        assert rdm2.segment.data.atmospheric_reentry_parameters.orbit_lifetime == 5.0

    def test_roundtrip_xml(self):
        try:
            rdm = self._create_valid_rdm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        xml = rdm.to_str(format="xml")
        assert "<rdm" in xml

        rdm2 = Rdm.from_str(xml, format="xml")
        assert rdm2.segment.data.atmospheric_reentry_parameters.orbit_lifetime == 5.0

    def test_file_io(self, tmp_path):
        rdm = self._create_valid_rdm()
        path = tmp_path / "test.rdm"

        rdm.to_file(str(path), format="kvn")
        assert path.exists()

        rdm2 = Rdm.from_file(str(path), format="kvn")
        assert rdm2.header.originator == "TEST"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
