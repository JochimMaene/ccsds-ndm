# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Attitude Comprehensive Message (ACM) Python bindings.
"""

import pytest
from ccsds_ndm import (
    Acm,
    AcmAttitudeState,
    AcmData,
    AcmMetadata,
    AcmSegment,
    AdmHeader,
)


class TestAcm:
    """Tests for ACM bindings."""

    def test_acm_metadata(self):
        # AcmMetadata signature: object_name, epoch_tzero, time_system, international_designator, comment
        meta = AcmMetadata(
            object_name="SAT1",
            epoch_tzero="2023-01-01T00:00:00",
            time_system="UTC",
            international_designator="2023-001A",
        )
        assert meta.object_name == "SAT1"
        assert meta.international_designator == "2023-001A"

    def _create_valid_acm(self):
        # Create a minimal valid ACM
        header = AdmHeader(
            classification="UNCLASSIFIED",
            creation_date="2023-01-01T00:00:00",
            originator="TEST",
            message_id="ID",
            comment=[],
        )
        meta = AcmMetadata(
            object_name="SAT1",
            epoch_tzero="2023-01-01T00:00:00",
            time_system="UTC",
            international_designator="2023-001A",
        )

        # AcmAttitudeState: ref_frame_a, ref_frame_b, att_type, att_lines, comment
        # att_lines is Vec<Vec<f64>> (each inner vec is values for AttLine)
        # AttLine values depend on type. Simplified context.
        state = AcmAttitudeState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            att_type="QUATERNION",
            att_lines=[[0.0, 0.0, 0.0, 1.0]],  # Single line with quaternion
            comment=[],
        )

        # AcmData: att, phys, cov, man, ad, user
        data = AcmData(att=[state], phys=None, cov=None, man=None, ad=None, user=None)

        segment = AcmSegment(metadata=meta, data=data)
        return Acm(header=header, segment=segment)

    def test_roundtrip_kvn(self):
        acm = self._create_valid_acm()
        kvn = acm.to_str(format="kvn")
        assert "CCSDS_ACM_VERS" in kvn

        acm2 = Acm.from_str(kvn, format="kvn")
        assert acm2.header.originator == "TEST"
        assert acm2.segment.metadata.object_name == "SAT1"

    def test_roundtrip_xml(self):
        acm = self._create_valid_acm()
        xml = acm.to_str(format="xml")
        assert "<acm" in xml

        acm2 = Acm.from_str(xml, format="xml")
        assert acm2.header.originator == "TEST"

    def test_file_io(self, tmp_path):
        acm = self._create_valid_acm()
        kvn_path = tmp_path / "test.acm"

        # Test to_file
        acm.to_file(str(kvn_path), format="kvn")
        assert kvn_path.exists()

        # Test from_file
        acm2 = Acm.from_file(str(kvn_path), format="kvn")
        assert acm2.header.originator == "TEST"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
