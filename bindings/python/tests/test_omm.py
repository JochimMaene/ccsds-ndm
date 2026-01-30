# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Orbit Mean-Elements Message (OMM) Python bindings.
"""

import pytest
from ccsds_ndm import MeanElements, OdmHeader, Omm, OmmData, OmmMetadata, OmmSegment


class TestOmm:
    """Tests for OMM bindings."""

    def _create_valid_omm(self):
        header = OdmHeader("2023-01-01T00:00:00", "TEST", "UNCLASSIFIED", "ID", [])

        meta = OmmMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            center_name="EARTH",
            ref_frame="TEME",
            time_system="UTC",
            mean_element_theory="DSST",
        )

        # MeanElements signature:
        # (epoch, eccentricity, inclination, ra_of_asc_node, arg_of_pericenter, mean_anomaly, semi_major_axis, mean_motion, gm)
        mean = MeanElements(
            epoch="2023-01-01T00:00:00",
            eccentricity=0.001,
            inclination=98.0,
            ra_of_asc_node=10.0,
            arg_of_pericenter=20.0,
            mean_anomaly=30.0,
            semi_major_axis=7000.0,
            mean_motion=None,
            gm=398600.44,
        )

        data = OmmData(mean_elements=mean, comments=[])

        seg = OmmSegment(metadata=meta, data=data)

        return Omm(header=header, segment=seg)

    def test_roundtrip_kvn(self):
        try:
            omm = self._create_valid_omm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        kvn = omm.to_str(format="kvn")
        assert "CCSDS_OMM_VERS" in kvn

        omm2 = Omm.from_str(kvn, format="kvn")
        assert omm2.header.originator == "TEST"
        assert omm2.segment.data.mean_elements.eccentricity == 0.001

    def test_roundtrip_xml(self):
        try:
            omm = self._create_valid_omm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        xml = omm.to_str(format="xml")
        assert "<omm" in xml

        omm2 = Omm.from_str(xml, format="xml")
        assert omm2.segment.data.mean_elements.eccentricity == 0.001

    def test_file_io(self, tmp_path):
        omm = self._create_valid_omm()
        path = tmp_path / "test.omm"

        omm.to_file(str(path), format="kvn")
        assert path.exists()

        omm2 = Omm.from_file(str(path), format="kvn")
        assert omm2.header.originator == "TEST"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
