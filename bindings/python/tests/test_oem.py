# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Orbit Ephemeris Message (OEM) Python bindings.
"""

import numpy as np
import pytest
from ccsds_ndm import (
    OdmHeader,
    Oem,
    OemCovarianceMatrix,
    OemData,
    OemMetadata,
    OemSegment,
    StateVectorAcc,
)


class TestOem:
    """Tests for OEM bindings."""

    def _create_valid_oem(self):
        header = OdmHeader("2023-01-01T00:00:00", "TEST", "UNCLASSIFIED", "ID", None)
        meta = OemMetadata(
            "SAT1",
            "2023-001A",
            "2023-01-01T00:00:00",
            "2023-01-01T01:00:00",
            center_name="EARTH",
            ref_frame="EME2000",
            time_system="UTC",
        )

        vec = StateVectorAcc(
            epoch="2023-01-01T00:00:00",
            x=7000.0,
            y=0.0,
            z=0.0,
            x_dot=0.0,
            y_dot=7.5,
            z_dot=0.0,
            x_ddot=None,
            y_ddot=None,
            z_ddot=None,
        )

        # OemCovarianceMatrix construction
        # Using 21 floats for 6x6 lower triangle
        cov_args = np.array([1.0] * 21, dtype=float)
        cov = OemCovarianceMatrix("2023-01-01T00:00:00", cov_args, "EME2000", [])

        data = OemData(state_vectors=[vec], comments=None)
        data.covariance_matrix = [cov]

        seg = OemSegment(meta, data)
        return Oem(header, [seg])

    def test_roundtrip_kvn(self):
        try:
            oem = self._create_valid_oem()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        kvn = oem.to_str(format="kvn")
        assert "CCSDS_OEM_VERS" in kvn

        oem2 = Oem.from_str(kvn, format="kvn")
        assert oem2.header.originator == "TEST"
        assert len(oem2.segments) == 1

    def test_roundtrip_xml(self):
        try:
            oem = self._create_valid_oem()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        xml = oem.to_str(format="xml")
        assert "<oem" in xml

        oem2 = Oem.from_str(xml, format="xml")
        assert len(oem2.segments) == 1

    def test_file_io(self, tmp_path):
        oem = self._create_valid_oem()
        path = tmp_path / "test.oem"

        oem.to_file(str(path), format="kvn")
        assert path.exists()

        oem2 = Oem.from_file(str(path), format="kvn")
        assert oem2.header.originator == "TEST"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
