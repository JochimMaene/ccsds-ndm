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

    def test_ref_frame_epoch_requires_calendar_form(self):
        with pytest.raises(ValueError):
            OmmMetadata(
                object_name="SAT1",
                object_id="2023-001A",
                center_name="EARTH",
                ref_frame="TEME",
                time_system="UTC",
                mean_element_theory="DSST",
                ref_frame_epoch="123.5",
            )

        omm = self._create_valid_omm()
        with pytest.raises(ValueError):
            omm.segment.data.mean_elements.epoch = "123.5"

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

    def test_from_tle_lines_rejects_non_uppercase_launch_piece(self):
        line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845"
        line1_lower = (
            "1 25544U 98067a   20348.69171878  .00000888  00000-0  24124-4 0  9995"
        )

        with pytest.raises(ValueError):
            Omm.from_tle_lines(line1_lower, line2)

    def test_from_tle_lines_accepts_missing_checksums(self):
        line1 = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  999"
        line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.4918115325984"
        assert len(line1) == 68
        assert len(line2) == 68

        omm = Omm.from_tle_lines(line1, line2)
        out1, out2 = omm.to_tle_lines()
        assert (
            out1
            == "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995"
        )
        assert (
            out2
            == "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845"
        )

    def test_to_tle_lines_accepts_sgp_slash_sgp4(self):
        kvn = """CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2020-065T16:00:00
ORIGINATOR = NOAA
MESSAGE_ID = OMM 202013719185
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP/SGP4
EPOCH = 2020-064T10:34:41.4264
MEAN_MOTION = 1.00273272
ECCENTRICITY = 0.0005013
INCLINATION = 3.0539
RA_OF_ASC_NODE = 81.7939
ARG_OF_PERICENTER = 249.2363
MEAN_ANOMALY = 150.1602
GM = 398600.8
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 23581
ELEMENT_SET_NO = 0925
REV_AT_EPOCH = 4316
BSTAR = 0.0001
MEAN_MOTION_DOT = -0.00000113
MEAN_MOTION_DDOT = 0.0
"""
        omm = Omm.from_str(kvn, format="kvn")
        line1, line2 = omm.to_tle_lines()
        assert line1.startswith("1 23581U 95025A")
        assert line2.startswith("2 23581")


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
