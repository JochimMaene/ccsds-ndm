# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Attitude Ephemeris Message (AEM) Python bindings.
"""

import numpy as np
import pytest
from ccsds_ndm import (
    AdmHeader,
    Aem,
    AemData,
    AemMetadata,
    AemSegment,
    AttitudeState,
)


class TestAem:
    """Tests for AEM bindings."""

    def test_aem_metadata(self):
        meta = AemMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            start_time="2023-01-01T00:00:00",
            stop_time="2023-01-01T01:00:00",
            attitude_type="QUATERNION",
            interpolation_method="LINEAR",
            interpolation_degree=1,
        )
        assert meta.object_name == "SAT1"
        assert meta.interpolation_method == "LINEAR"
        assert meta.interpolation_degree == 1

    def test_aem_data_numpy(self):
        # Create data using python list of states
        state1 = AttitudeState("2023-01-01T00:00:00", [0.0, 0.0, 0.0, 1.0])
        state2 = AttitudeState("2023-01-01T00:01:00", [0.0, 0.0, 0.0, 1.0])
        data = AemData(attitude_states=[state1, state2], comment=[])

        # Test getting as numpy
        epochs, array = data.attitude_states_numpy  # Note: Property access
        assert len(epochs) == 2
        assert array.shape == (2, 4)
        assert array[0, 3] == 1.0

        # Test setting from numpy
        new_epochs = ["2023-01-01T00:00:00", "2023-01-01T00:01:00"]
        new_array = np.array([[0.5, 0.5, 0.5, 0.5], [0.1, 0.1, 0.1, 0.1]])
        data.attitude_states_numpy = (new_epochs, new_array)

        # Verify update
        states = data.attitude_states
        assert len(states) == 2
        # Check tolerance or exact value
        assert abs(states[0].values[0] - 0.5) < 1e-9

    def _create_valid_aem(self):
        header = AdmHeader(
            classification="UNCLASSIFIED",
            creation_date="2023-01-01T00:00:00",
            originator="TEST",
            message_id="ID",
            comment=[],
        )
        meta = AemMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            start_time="2023-01-01T00:00:00",
            stop_time="2023-01-01T01:00:00",
            attitude_type="QUATERNION",
        )
        state1 = AttitudeState("2023-01-01T00:00:00", [0.0, 0.0, 0.0, 1.0])
        data = AemData(attitude_states=[state1], comment=[])
        segment = AemSegment(meta, data)
        return Aem(header, [segment])

    def test_roundtrip_kvn(self):
        aem = self._create_valid_aem()
        kvn = aem.to_str(format="kvn")
        assert "CCSDS_AEM_VERS" in kvn

        aem2 = Aem.from_str(kvn, format="kvn")
        assert aem2.header.originator == "TEST"
        assert len(aem2.segments) == 1
        assert aem2.segments[0].data.attitude_states[0].values[3] == 1.0

    def test_roundtrip_xml(self):
        aem = self._create_valid_aem()
        xml = aem.to_str(format="xml")
        assert "<aem" in xml

        aem2 = Aem.from_str(xml, format="xml")
        assert aem2.header.originator == "TEST"
        assert len(aem2.segments) == 1

    def test_file_io(self, tmp_path):
        aem = self._create_valid_aem()
        kvn_path = tmp_path / "test.aem"

        # Write to file directly
        aem.to_file(str(kvn_path), format="kvn")
        assert kvn_path.exists()

        # Read back
        aem2 = Aem.from_file(str(kvn_path), format="kvn")
        assert aem2.header.originator == "TEST"

    def test_construction(self):
        aem = self._create_valid_aem()
        assert aem.header.originator == "TEST"
        assert len(aem.segments) == 1


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
