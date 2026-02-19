# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Orbit Comprehensive Message (OCM) Python bindings.
"""

import pytest
from ccsds_ndm import (
    CovLine,
    ManLine,
    Ocm,
    OcmCovarianceMatrix,
    OcmData,
    OcmManeuverParameters,
    OcmMetadata,
    OcmSegment,
    OcmTrajState,
    OdmHeader,
    TrajLine,
)


class TestOcm:
    """Tests for OCM bindings."""

    def _create_valid_ocm(self):
        header = OdmHeader("2023-01-01T00:00:00", "TEST", "UNCLASSIFIED", "ID", [])

        # OcmMetadata requires epoch_tzero keyword-only (pyo3 signature) or positional?
        # ocm.rs: epoch_tzero is first argued after *, so strictly keyword?
        # signature = (*, epoch_tzero, ...)
        # So I MUST use keyword args.
        meta = OcmMetadata(
            epoch_tzero="2023-01-01T00:00:00",
            object_name="SAT1",
            international_designator="2023-001A",
            time_system="UTC",
        )

        traj_line = TrajLine(
            epoch="2023-01-01T00:00:00", values=[7000.0, 0.0, 0.0, 0.0, 7.5, 0.0]
        )

        traj = OcmTrajState(
            center_name="EARTH",
            traj_ref_frame="EME2000",
            traj_type="CARTPV",
            traj_lines=[traj_line],
        )

        data = OcmData()
        data.traj = [traj]

        seg = OcmSegment(meta, data)
        return Ocm(header, seg)

    def test_roundtrip_kvn(self):
        try:
            ocm = self._create_valid_ocm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        kvn = ocm.to_str(format="kvn")
        assert "CCSDS_OCM_VERS" in kvn

        ocm2 = Ocm.from_str(kvn, format="kvn")
        assert ocm2.header.originator == "TEST"
        assert len(ocm2.segment.data.traj) == 1

    def test_roundtrip_xml(self):
        try:
            ocm = self._create_valid_ocm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        xml = ocm.to_str(format="xml")
        assert "<ocm" in xml

        ocm2 = Ocm.from_str(xml, format="xml")
        assert len(ocm2.segment.data.traj) == 1

    def test_file_io(self, tmp_path):
        ocm = self._create_valid_ocm()
        path = tmp_path / "test.ocm"

        ocm.to_file(str(path), format="kvn")
        assert path.exists()

        ocm2 = Ocm.from_file(str(path), format="kvn")
        assert ocm2.header.originator == "TEST"

    def test_ocm_covariance_and_maneuver_setters(self):
        cov = OcmCovarianceMatrix(
            cov_ref_frame="EME2000",
            cov_type="CARTPV",
            cov_ordering="LTM",
            cov_lines=[CovLine(epoch="0.0", values=[1.0])],
        )
        cov.cov_basis = "PREDICTED"
        assert cov.cov_basis is not None
        assert cov.cov_basis.lower().startswith("pred")

        cov.cov_confidence = 42.5
        assert cov.cov_confidence == pytest.approx(42.5)

        cov.cov_ordering = "UTM"
        assert cov.cov_ordering.lower().startswith("utm")

        with pytest.raises(ValueError):
            cov.cov_basis = "NOT_A_BASIS"
        with pytest.raises(ValueError):
            cov.cov_ordering = "NOT_A_ORDERING"

        man = OcmManeuverParameters(
            man_id="MAN-1",
            man_device_id="THR-1",
            man_composition="VECTOR",
            man_ref_frame="EME2000",
            man_lines=[
                ManLine(epoch="2023-01-01T00:00:00", values=["0.1", "0.0", "0.0"])
            ],
        )
        man.man_basis = "PLANNED"
        assert man.man_basis is not None
        assert man.man_basis.lower().startswith("plan")

        with pytest.raises(ValueError):
            man.man_basis = "NOT_A_BASIS"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
