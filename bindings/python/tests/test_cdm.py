# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Conjunction Data Message (CDM) Python bindings.
"""

import pathlib

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
    RelativeMetadataData,
)


def test_cdm():
    cdm = Cdm.from_file(
        str(pathlib.Path(__file__).parent / "cdm_spacetrack1.xml"), format="xml"
    )
    print(cdm)


class TestCdm:
    """Tests for CDM bindings."""

    def _create_valid_cdm(self):
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
            # Strict mode requires defaults or we assumed signature handles None?
            # RelativeMetadataData has pyo3 signature defaults, so this should be fine.
        )

        # Object 1
        meta1 = CdmMetadata(
            object="OBJECT1",
            object_designator="12345",
            catalog_name="SATCAT",
            object_name="SAT1",
            international_designator="2023-001A",
            ref_frame="EME2000",
        )
        vector1 = CdmStateVector(
            x=7000.0, y=0.0, z=0.0, x_dot=0.0, y_dot=7.5, z_dot=0.0
        )

        # CdmCovarianceMatrix: 21 mandatory floats (6x6 lower tri), 24 optional floats (7-9), comment
        cov_args = [
            # 6x6 Lower Triangle (21 elements)
            # R, T, N order usually? Rust struct fields: cr_r, ct_r, ct_t, ...
            # Row 1: cr_r
            1.0,
            # Row 2: ct_r, ct_t
            0.0,
            1.0,
            # Row 3: cn_r, cn_t, cn_n
            0.0,
            0.0,
            1.0,
            # Row 4: crdot_r... crdot_rdot
            0.0,
            0.0,
            0.0,
            0.01,
            # Row 5: ctdot_r... ctdot_tdot
            0.0,
            0.0,
            0.0,
            0.0,
            0.01,
            # Row 6: cndot_r... cndot_ndot
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.01,
            # Rows 7, 8, 9 (24 optionals) -> All None
            *[None] * 24,
            # Comment
            None,
        ]

        cov1 = CdmCovarianceMatrix(*cov_args)

        data1 = CdmData(
            state_vector=vector1,
            covariance_matrix=cov1,
            comments=[],  # Mandatory positional
        )
        seg1 = CdmSegment(metadata=meta1, data=data1)

        # Object 2
        meta2 = CdmMetadata(
            object="OBJECT2",
            object_designator="67890",
            catalog_name="SATCAT",
            object_name="DEBRIS",
            international_designator="UNKNOWN",
            ref_frame="EME2000",
        )
        vector2 = CdmStateVector(
            x=7100.0, y=0.0, z=0.0, x_dot=0.0, y_dot=7.4, z_dot=0.0
        )
        cov2 = CdmCovarianceMatrix(*cov_args)
        data2 = CdmData(state_vector=vector2, covariance_matrix=cov2, comments=[])
        seg2 = CdmSegment(metadata=meta2, data=data2)

        body = CdmBody(relative_metadata_data=rel_meta, segments=[seg1, seg2])

        return Cdm(header=header, body=body, id="CDM-ID", version="1.0")

    def test_roundtrip_kvn(self):
        try:
            cdm = self._create_valid_cdm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        kvn = cdm.to_str(format="kvn")
        assert "CCSDS_CDM_VERS" in kvn

        cdm2 = Cdm.from_str(kvn, format="kvn")
        assert cdm2.header.originator == "TEST"
        assert len(cdm2.body.segments) == 2

    def test_roundtrip_xml(self):
        try:
            cdm = self._create_valid_cdm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        xml = cdm.to_str(format="xml")
        assert "<cdm" in xml

        cdm2 = Cdm.from_str(xml, format="xml")
        assert cdm2.header.originator == "TEST"

    def test_file_io(self, tmp_path):
        cdm = self._create_valid_cdm()
        kvn_path = tmp_path / "test.cdm"

        # Test to_file
        cdm.to_file(str(kvn_path), format="kvn")
        assert kvn_path.exists()

        # Test from_file
        cdm2 = Cdm.from_file(str(kvn_path), format="kvn")
        assert cdm2.header.originator == "TEST"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
