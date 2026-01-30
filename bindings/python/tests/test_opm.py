# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Orbit Parameter Message (OPM) Python bindings.
"""

import pytest
from ccsds_ndm import (
    KeplerianElements,
    OdmHeader,
    Opm,
    OpmData,
    OpmMetadata,
    OpmSegment,
    StateVector,
)


class TestOpm:
    """Tests for OPM bindings."""

    def _create_valid_opm(self):
        header = OdmHeader("2023-01-01T00:00:00", "TEST", "UNCLASSIFIED", "ID", [])

        meta = OpmMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            center_name="EARTH",
            ref_frame="GCRF",
            time_system="UTC",
        )

        # StateVector signature needs comments=None explicitly
        state = StateVector(
            epoch="2023-01-01T00:00:00",
            x=7000.0,
            y=0.0,
            z=0.0,
            x_dot=0.0,
            y_dot=7.5,
            z_dot=0.0,
            comments=None,
        )

        # KeplerianElements signature:
        kepler = KeplerianElements(
            semi_major_axis=7000.0,
            eccentricity=0.001,
            inclination=98.0,
            ra_of_asc_node=10.0,
            arg_of_pericenter=20.0,
            gm=398600.44,
            true_anomaly=0.0,
            mean_anomaly=None,
        )

        data = OpmData(state_vector=state, comment=[])
        data.keplerian_elements = kepler

        seg = OpmSegment(metadata=meta, data=data)

        return Opm(header=header, segment=seg)

    def test_roundtrip_kvn(self):
        try:
            opm = self._create_valid_opm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        kvn = opm.to_str(format="kvn")
        assert "CCSDS_OPM_VERS" in kvn

        opm2 = Opm.from_str(kvn, format="kvn")
        assert opm2.header.originator == "TEST"
        assert opm2.segment.data.state_vector.x == 7000.0

    def test_roundtrip_xml(self):
        try:
            opm = self._create_valid_opm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        xml = opm.to_str(format="xml")
        assert "<opm" in xml

        opm2 = Opm.from_str(xml, format="xml")
        assert opm2.segment.data.state_vector.x == 7000.0

    def test_file_io(self, tmp_path):
        opm = self._create_valid_opm()
        path = tmp_path / "test.opm"

        opm.to_file(str(path), format="kvn")
        assert path.exists()

        opm2 = Opm.from_file(str(path), format="kvn")
        assert opm2.header.originator == "TEST"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
