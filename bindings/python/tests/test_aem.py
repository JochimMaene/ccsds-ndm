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

    def test_adm_creation_date_requires_absolute_epoch(self):
        with pytest.raises(ValueError):
            AdmHeader("123.5", "TEST")

        header = AdmHeader("2002-204T15:56:23Z", "TEST")
        assert header.creation_date == "2002-204T15:56:23Z"
        with pytest.raises(ValueError):
            header.creation_date = "123.5"

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

    def test_aem_metadata_invalid_attitude_type_raises(self):
        with pytest.raises(ValueError):
            AemMetadata(
                object_name="SAT1",
                object_id="2023-001A",
                ref_frame_a="EME2000",
                ref_frame_b="SC_BODY_1",
                start_time="2023-01-01T00:00:00",
                stop_time="2023-01-01T01:00:00",
                attitude_type="NOT_IN_XSD_ENUM",
            )

    def test_attitude_state_setters(self):
        state = AttitudeState("2023-01-01T00:00:00", [0.0, 0.0, 0.0, 1.0])
        assert state.epoch == "2023-01-01T00:00:00"
        assert state.values == [0.0, 0.0, 0.0, 1.0]

        state.epoch = "2023-01-01T00:01:00"
        state.values = [0.1, 0.2, 0.3, 0.4]
        assert state.epoch == "2023-01-01T00:01:00"
        assert state.values == [0.1, 0.2, 0.3, 0.4]

        with pytest.raises(ValueError):
            state.epoch = "not-an-epoch"

        with pytest.raises(ValueError):
            AttitudeState("123.5", [0.0, 0.0, 0.0, 1.0])

        meta = AemMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            start_time="2023-01-01T00:00:00",
            stop_time="2023-01-01T01:00:00",
        )
        with pytest.raises(ValueError):
            meta.start_time = "123.5"

    def test_aem_data_numpy(self):
        # Create data using python list of states
        state1 = AttitudeState("2023-01-01T00:00:00", [0.0, 0.0, 0.0, 1.0])
        state2 = AttitudeState("2023-01-01T00:01:00", [0.0, 0.0, 0.0, 1.0])
        data = AemData(
            attitude_states=[state1, state2],
            attitude_type="QUATERNION",
            comment=[],
        )

        # Test getting as numpy
        epochs = data.attitude_states_epochs  # Note: Property access
        array = data.attitude_states_numpy
        assert len(epochs) == 2
        assert array.shape == (2, 4)
        assert array[0, 3] == 1.0

        # Test setting from numpy
        new_epochs = ["2023-01-01T00:00:00", "2023-01-01T00:01:00"]
        new_array = np.array([[0.5, 0.5, 0.5, 0.5], [0.1, 0.1, 0.1, 0.1]])
        data.attitude_states_epochs = new_epochs
        data.attitude_states_numpy = new_array

        # Verify update
        states = data.attitude_states
        assert len(states) == 2
        # Check tolerance or exact value
        assert abs(states[0].values[0] - 0.5) < 1e-9

        # Test from_numpy constructor
        data2 = AemData.from_numpy(
            new_epochs, new_array, attitude_type="QUATERNION", comment=[]
        )
        assert len(data2.attitude_states) == 2

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
        data = AemData(attitude_states=[state1], attitude_type="QUATERNION", comment=[])
        segment = AemSegment(meta, data)
        return Aem(header, [segment])

    def test_aem_from_numpy_requires_explicit_type_for_ambiguous_width(self):
        epochs = ["2023-01-01T00:00:00"]
        values = np.array([[0.0, 0.0, 0.0, 1.0]])

        with pytest.raises(ValueError, match="Ambiguous 4-column AEM data"):
            AemData.from_numpy(epochs, values, comment=[])

        values_6 = np.array([[1.0, 2.0, 3.0, 0.1, 0.2, 0.3]])
        with pytest.raises(ValueError, match="Ambiguous 6-column AEM data"):
            AemData.from_numpy(epochs, values_6, comment=[])

    def test_aem_from_numpy_rejects_wrong_width_without_defaults(self):
        epochs = ["2023-01-01T00:00:00"]
        values = np.array([[0.1, 0.2]])

        with pytest.raises(ValueError, match="requires 4 columns"):
            AemData.from_numpy(epochs, values, attitude_type="QUATERNION", comment=[])

    def test_aem_from_numpy_spin_supported(self):
        epochs = ["2023-01-01T00:00:00"]
        # SPIN: SPIN_ALPHA, SPIN_DELTA, SPIN_ANGLE, SPIN_ANGLE_VEL
        values = np.array([[10.0, 20.0, 30.0, 0.5]])

        data = AemData.from_numpy(epochs, values, attitude_type="SPIN", comment=[])
        states = data.attitude_states
        assert len(states) == 1
        assert states[0].epoch == "2023-01-01T00:00:00"
        assert states[0].values == [10.0, 20.0, 30.0, 0.5]

    def test_aem_set_epochs_without_states_raises(self):
        data = AemData(attitude_states=[], comment=[])
        with pytest.raises(ValueError, match="Cannot set epochs"):
            data.attitude_states_epochs = ["2023-01-01T00:00:00"]

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
