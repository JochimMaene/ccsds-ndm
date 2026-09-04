# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Orbit Ephemeris Message (OEM) Python bindings.
"""

import ccsds_ndm
import numpy as np
import pytest

from ccsds_ndm import (
    NdmValidationError,
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

    def test_odm_creation_date_requires_absolute_epoch(self):
        with pytest.raises(ValueError):
            OdmHeader("123.5", "TEST")

        header = OdmHeader("2002-204T15:56:23Z", "TEST")
        assert header.creation_date == "2002-204T15:56:23Z"
        with pytest.raises(ValueError):
            header.creation_date = "123.5"

    def test_ref_frame_epoch_requires_calendar_form(self):
        with pytest.raises(ValueError):
            OemMetadata(
                "SAT1",
                "2023-001A",
                "2023-01-01T00:00:00",
                "2023-01-01T01:00:00",
                center_name="EARTH",
                ref_frame="EME2000",
                time_system="UTC",
                ref_frame_epoch="123.5",
            )

        metadata = self._create_valid_oem().segments[0].metadata
        metadata.ref_frame_epoch = "2000-001T12:00:00"
        assert metadata.ref_frame_epoch == "2000-001T12:00:00"
        with pytest.raises(ValueError):
            metadata.ref_frame_epoch = "123.5"

    def test_contextual_epoch_validation_rejects_degenerate_values(self):
        metadata = self._create_valid_oem().segments[0].metadata
        metadata.start_time = "+"
        with pytest.raises(ValueError):
            metadata.validate()

        oem = self._create_valid_oem()
        segments = oem.segments
        data = segments[0].data
        state_vectors = data.state_vector
        state_vectors[0].epoch = "2023-02-29T00:00:00"
        data.state_vector = state_vectors
        segments[0].data = data
        oem.segments = segments
        with pytest.raises(NdmValidationError):
            oem.to_str(format="xml")

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

        oem.to_file(str(path), "kvn")
        assert path.exists()

        oem2 = ccsds_ndm.from_file(str(path), format="kvn")
        assert oem2.header.originator == "TEST"

    def test_oem_data_numpy_api(self):
        epochs = ["2023-01-01T00:00:00", "2023-01-01T00:01:00"]
        state = np.array(
            [
                [7000.0, 0.0, 0.0, 0.0, 7.5, 0.0],
                [7001.0, 0.1, 0.2, 0.0, 7.5, 0.0],
            ],
            dtype=float,
        )
        cov_epochs = ["2023-01-01T00:00:00"]
        cov = np.eye(6, dtype=float).reshape(1, 6, 6)

        data = OemData.from_numpy(
            state_vector_epochs=epochs,
            state_vector_numpy=state,
            covariance_matrix_epochs=cov_epochs,
            covariance_matrix_numpy=cov,
            comments=[],
        )

        assert data.state_vector_epochs == epochs
        assert data.state_vector_numpy.shape == (2, 6)
        assert data.covariance_matrix_epochs == cov_epochs
        assert data.covariance_matrix_numpy.shape == (1, 6, 6)

        new_state = state + 1.0
        data.state_vector_numpy = new_state
        assert np.allclose(data.state_vector_numpy, new_state)

    def test_full_covariance_inputs_read_the_lower_triangle(self):
        # Filter output is symmetric only to within rounding, so the upper triangle is ignored
        # rather than compared for equality.
        asymmetric = np.eye(6, dtype=float)
        asymmetric[0, 1] = 1.0

        epochs = ["2023-01-01T00:00:00"]
        state = np.zeros((1, 6), dtype=float)
        data = OemData.from_numpy(
            state_vector_epochs=epochs,
            state_vector_numpy=state,
            covariance_matrix_epochs=epochs,
            covariance_matrix_numpy=asymmetric.reshape(1, 6, 6),
        )
        assert np.allclose(data.covariance_matrix_numpy[0], np.eye(6, dtype=float))

        matrix = OemCovarianceMatrix("2023-01-01T00:00:00", asymmetric, None, [])
        assert matrix.cx_x == 1.0
        assert matrix.cy_x == 0.0

        nearly = np.eye(6, dtype=float)
        nearly[1, 0] = 1e-17
        data.covariance_matrix_numpy = nearly.reshape(1, 6, 6)
        assert np.allclose(data.covariance_matrix_numpy[0], np.eye(6, dtype=float))


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
