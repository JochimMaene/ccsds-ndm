# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Tracking Data Message (TDM) Python bindings.
"""

import pytest
from ccsds_ndm import (
    Tdm,
    TdmBody,
    TdmData,
    TdmHeader,
    TdmMetadata,
    TdmObservation,
    TdmSegment,
)


class TestTdm:
    """Tests for TDM bindings."""

    def _create_valid_tdm(self):
        header = TdmHeader(
            originator="TEST", creation_date="2023-01-01T00:00:00", message_id="ID"
        )

        meta = TdmMetadata(
            participant_1="STN",
            participant_2="SAT",
            time_system="UTC",
            mode="SEQUENTIAL",
            path="1,2,1",
        )

        obs = TdmObservation(epoch="2023-01-01T00:00:00", keyword="RANGE", value=1000.0)
        # Using TdmData with observations list
        data = TdmData(observations=[obs])

        seg = TdmSegment(metadata=meta, data=data)

        body = TdmBody(segments=[seg])

        return Tdm(header=header, body=body)

    def test_roundtrip_kvn(self):
        try:
            tdm = self._create_valid_tdm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        kvn = tdm.to_str(format="kvn")
        assert "CCSDS_TDM_VERS" in kvn

        tdm2 = Tdm.from_str(kvn, format="kvn")
        assert tdm2.header.originator == "TEST"
        assert len(tdm2.body.segments) == 1
        obs = tdm2.body.segments[0].data.observations[0]
        assert obs.keyword == "RANGE"
        assert obs.value == 1000.0

    def test_roundtrip_xml(self):
        try:
            tdm = self._create_valid_tdm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        xml = tdm.to_str(format="xml")
        assert "<tdm" in xml

        tdm2 = Tdm.from_str(xml, format="xml")
        assert len(tdm2.body.segments) == 1
        obs = tdm2.body.segments[0].data.observations[0]
        assert obs.keyword == "RANGE"

    def test_file_io(self, tmp_path):
        tdm = self._create_valid_tdm()
        path = tmp_path / "test.tdm"

        tdm.to_file(str(path), format="kvn")
        assert path.exists()

        tdm2 = Tdm.from_file(str(path), format="kvn")
        assert tdm2.header.originator == "TEST"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
