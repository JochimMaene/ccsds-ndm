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

    def test_tle_lines_roundtrip(self):
        line1 = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995"
        line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845"

        omm = Omm.from_tle_lines(
            line1,
            line2,
            object_name="ISS (ZARYA)",
            originator="18 SPCS",
        )

        out1, out2 = omm.to_tle_lines()
        assert out1 == line1
        assert out2 == line2

    def test_from_tle_lines_defaults(self):
        line1 = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995"
        line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845"

        omm = Omm.from_tle_lines(line1, line2)
        assert omm.segment.metadata.object_name == "UNKNOWN"
        assert omm.segment.metadata.object_id == "1998-067A"
        assert omm.segment.metadata.center_name == "EARTH"
        assert omm.segment.metadata.ref_frame == "TEME"
        assert omm.segment.metadata.time_system == "UTC"
        assert omm.segment.metadata.mean_element_theory == "SGP4"
        assert omm.segment.data.tle_parameters.norad_cat_id == 25544


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
