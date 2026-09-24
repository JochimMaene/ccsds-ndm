# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Attitude Ephemeris Message (AEM) Python bindings.
"""

from pathlib import Path

import ccsds_ndm
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

KVN_DIR = Path(__file__).resolve().parents[3] / "ccsds-ndm" / "data" / "kvn"


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
            time_system="UTC",
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
                time_system="UTC",
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
            time_system="UTC",
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
            time_system="UTC",
            attitude_type="QUATERNION",
        )
        state1 = AttitudeState("2023-01-01T00:00:00", [0.0, 0.0, 0.0, 1.0])
        data = AemData(attitude_states=[state1], attitude_type="QUATERNION", comment=[])
        segment = AemSegment(meta, data)
        return Aem(header, [segment])

    def test_aem_from_numpy_requires_explicit_type(self):
        epochs = ["2023-01-01T00:00:00"]
        values = np.array([[0.0, 0.0, 0.0, 1.0]])

        with pytest.raises(TypeError):
            AemData.from_numpy(epochs, values, comment=[])

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

    def test_attitude_states_epochs_reject_bad_input_without_partial_writes(self):
        state1 = AttitudeState("2023-01-01T00:00:00", [0.0, 0.0, 0.0, 1.0])
        state2 = AttitudeState("2023-01-01T00:01:00", [0.0, 0.0, 0.0, 1.0])
        data = AemData(
            attitude_states=[state1, state2],
            attitude_type="QUATERNION",
            comment=[],
        )
        original = data.attitude_states[0].epoch

        with pytest.raises(ValueError):
            data.attitude_states_epochs = [
                "2024-01-01T00:00:00",
                "not-a-timestamp",
            ]
        assert data.attitude_states[0].epoch == original

        data.attitude_states[1] = "not an attitude state"
        with pytest.raises(TypeError, match=r"attitude_states\[1\]"):
            data.attitude_states_epochs = [
                "2024-01-01T00:00:00",
                "2024-01-01T00:01:00",
            ]
        assert data.attitude_states[0].epoch == original

    def test_attitude_states_numpy_rejects_bad_record_without_partial_writes(self):
        state1 = AttitudeState("2023-01-01T00:00:00", [0.0, 0.0, 0.0, 1.0])
        state2 = AttitudeState("2023-01-01T00:01:00", [0.0, 0.0, 0.0, 1.0])
        data = AemData(
            attitude_states=[state1, state2],
            attitude_type="QUATERNION",
            comment=[],
        )

        # Row 0 used to be written before row 1's bad element raised.
        data.attitude_states[1] = "not an attitude state"
        with pytest.raises(
            TypeError, match=r"attitude_states\[1\] must be AttitudeState"
        ):
            data.attitude_states_numpy = np.full((2, 4), 0.5)
        assert state1.values == [0.0, 0.0, 0.0, 1.0]

        data.attitude_states[1] = state2
        data.attitude_states_numpy = np.full((2, 4), 0.5)
        assert data.attitude_states[0] is state1
        assert state1.values == [0.5] * 4

    def test_section_validation_raises_ndm_validation_error(self):
        aem = Aem.from_file(KVN_DIR / "aem_g5.kvn")
        segment = aem.segments[0]
        attitude_type = segment.metadata.attitude_type
        state = segment.data.attitude_states[0]
        state.values = [float("nan"), *state.values[1:]]
        with pytest.raises(ccsds_ndm.NdmValidationError):
            segment.data.validate(attitude_type)

        segment.metadata.object_name = ""
        for validate in (segment.metadata.validate, segment.validate):
            with pytest.raises(ccsds_ndm.NdmValidationError, match="OBJECT_NAME"):
                validate()

    def test_sections_apply_the_fraction_limit(self):
        # ADM 6.8.9 limits fractional seconds to 16 digits at every validation entry point.
        aem = Aem.from_file(KVN_DIR / "aem_g5.kvn")
        segment = aem.segments[0]
        whole, _, fraction = segment.metadata.start_time.partition(".")
        segment.metadata.start_time = f"{whole}.{fraction:0<17}"
        for validate in (segment.metadata.validate, segment.validate, aem.validate):
            with pytest.raises(ccsds_ndm.NdmValidationError, match="START_TIME"):
                validate()

    def test_aem_set_epochs_without_states_raises(self):
        data = AemData(attitude_states=[], attitude_type="QUATERNION", comment=[])
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

    def test_live_attitude_state_edits_are_revalidated_before_generation(self):
        aem = self._create_valid_aem()
        state = aem.segments[0].data.attitude_states[0]
        state.values = [float("nan"), 0.0, 0.0, 1.0]

        with pytest.raises(ccsds_ndm.NdmValidationError, match="Q1"):
            aem.to_str(format="xml")

    def test_file_io(self, tmp_path):
        aem = self._create_valid_aem()
        kvn_path = tmp_path / "test.aem"

        # Write to file directly
        aem.to_file(str(kvn_path), "kvn")
        assert kvn_path.exists()

        # Read back
        aem2 = ccsds_ndm.from_file(str(kvn_path), format="kvn")
        assert aem2.header.originator == "TEST"

    def test_construction(self):
        aem = self._create_valid_aem()
        assert aem.header.originator == "TEST"
        assert len(aem.segments) == 1

    def test_interpolation_degree_zero_is_rejected_not_dropped(self):
        with pytest.raises(ValueError, match="positive integer"):
            AemMetadata(
                object_name="SAT1",
                object_id="2023-001A",
                ref_frame_a="EME2000",
                ref_frame_b="SC_BODY_1",
                start_time="2023-01-01T00:00:00",
                stop_time="2023-01-01T01:00:00",
                time_system="UTC",
                interpolation_degree=0,
            )

        meta = self._create_valid_aem().segments[0].metadata
        meta.interpolation_degree = 5
        assert meta.interpolation_degree == 5
        with pytest.raises(ValueError, match="positive integer"):
            meta.interpolation_degree = 0
        assert meta.interpolation_degree == 5

        meta.interpolation_degree = None
        assert meta.interpolation_degree is None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
