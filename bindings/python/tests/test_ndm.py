# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Navigation Data Message (NDM) Python bindings.
"""

from pathlib import Path

import pytest

from ccsds_ndm import (
    Cdm,
    CdmBody,
    CdmCovarianceMatrix,
    CdmData,
    CdmHeader,
    CdmMetadata,
    CdmSegment,
    CdmStateVector,
    Ndm,
    NdmValidationError,
    OdmHeader,  # Correct header for OEM
    Oem,
    OemData,
    OemMetadata,
    OemSegment,
    RelativeMetadataData,
    StateVectorAcc,  # Verified class name
    Tdm,
)


class TestNdm:
    """Tests for NDM bindings."""

    def _create_valid_oem(self):
        header = OdmHeader(
            "2023-01-01T00:00:00",  # creation_date
            "TEST",  # originator
            "UNCLASSIFIED",  # classification
            "ID",  # message_id
            None,  # comment
        )
        # OemMetadata signature verified:
        meta = OemMetadata(
            "SAT1",  # object_name
            "2023-001A",  # object_id
            "2023-01-01T00:00:00",  # start_time
            "2023-01-01T01:00:00",  # stop_time
            "EARTH",  # center_name
            "EME2000",  # ref_frame
            time_system="UTC",  # keyword for clarity (ref_frame is positional arg 6 in rust, but python binds args positionally or keyword based on signature)
            # Rust signature: (object_name, object_id, start_time, stop_time, center_name, ref_frame, ...)
            # So ref_frame is 6th. "EME2000" passed as 6th positional arg above?
            # No, python call: "EARTH", "EME2000" (5th, 6th).
            # Wait, time_system is 7th. I used keyword. That's fine.
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

        data = OemData(state_vectors=[vec], comments=None)

        seg = OemSegment(meta, data)
        return Oem(header, [seg])

    def _create_valid_cdm(self):
        # reusing verified logic from test_cdm
        header = CdmHeader(
            creation_date="2023-01-01T00:00:00",
            originator="TEST",
            message_id="ID",
            message_for="SATELLITE",
            comment=[],
        )
        rel_meta = RelativeMetadataData(
            tca="2023-01-01T12:00:00",
            miss_distance=100.0,
            collision_probability=0.001,
            collision_probability_method="FOSTER-1992",
            miss_distance_unit="m",
        )

        meta1 = CdmMetadata(
            object="OBJECT1",
            object_designator="12345",
            catalog_name="SATCAT",
            object_name="SAT1",
            international_designator="2023-001A",
            ref_frame="EME2000",
        )
        vector1 = CdmStateVector(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0)
        cov_args = (
            [
                1.0,
                0.0,
                1.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.01,
                0.0,
                0.0,
                0.0,
                0.0,
                0.01,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.01,
            ]
            + [None] * 24
            + [None]
        )
        cov1 = CdmCovarianceMatrix(*cov_args)
        data1 = CdmData(vector1, cov1, comments=[])
        seg1 = CdmSegment(meta1, data1)

        meta2 = CdmMetadata(
            object="OBJECT2",
            object_designator="67890",
            catalog_name="SATCAT",
            object_name="DEBRIS",
            international_designator="UNKNOWN",
            ref_frame="EME2000",
        )
        vector2 = CdmStateVector(7100.0, 0.0, 0.0, 0.0, 7.4, 0.0)
        cov2 = CdmCovarianceMatrix(*cov_args)
        data2 = CdmData(vector2, cov2, comments=[])
        seg2 = CdmSegment(meta2, data2)

        body = CdmBody(rel_meta, [seg1, seg2])
        return Cdm(header, body)

    def test_accepts_tdm_message(self):
        root = Path(__file__).parents[3]
        tdm = Tdm.from_file(str(root / "ccsds-ndm/data/xml/tdm_e21.xml"), format="xml")

        combined = Ndm([tdm])

        assert isinstance(combined.messages[0], Tdm)

    def test_typed_ndm_rejects_single_message_kvn(self):
        root = Path(__file__).parents[3]
        opm = (root / "ccsds-ndm/data/kvn/opm_g1.kvn").read_text()

        for format_hint in (None, "kvn"):
            with pytest.raises(ValueError, match="different CCSDS NDM message type"):
                Ndm.from_str(opm, format=format_hint)

    def test_ndm_roundtrip_xml(self):
        oem = self._create_valid_oem()
        cdm = self._create_valid_cdm()

        ndm = Ndm(messages=[oem, cdm])

        xml = ndm.to_str(format="xml")
        assert "<ndm" in xml
        assert "<oem" in xml
        assert "<cdm" in xml

        ndm2 = Ndm.from_str(xml, format="xml")
        assert len(ndm2.messages) == 2

    def test_file_io(self, tmp_path):
        oem = self._create_valid_oem()
        ndm = Ndm([oem])
        path = tmp_path / "test.ndm"

        ndm.to_file(str(path), format="xml")
        assert path.exists()

        ndm2 = Ndm.from_file(str(path), format="xml")
        assert len(ndm2.messages) == 1

    def test_failed_file_generation_preserves_existing_file(self, tmp_path):
        oem = self._create_valid_oem()
        oem.version = "1.0"
        ndm = Ndm([oem])
        path = tmp_path / "test.ndm"
        path.write_text("keep me")

        with pytest.raises(NdmValidationError, match="output version 1.0") as caught:
            ndm.to_file(str(path), format="xml")

        assert caught.value.code == "generation.unsupported_output_version"
        assert path.read_text() == "keep me"

    def test_ndm_message_and_comment_setters(self):
        oem = self._create_valid_oem()
        cdm = self._create_valid_cdm()

        ndm = Ndm(messages=[oem], comments=["initial"])
        assert len(ndm.messages) == 1
        assert ndm.comments == ["initial"]

        ndm.messages = [oem, cdm]
        ndm.comments = ["updated", "suite"]

        assert len(ndm.messages) == 2
        assert ndm.comments == ["updated", "suite"]


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
