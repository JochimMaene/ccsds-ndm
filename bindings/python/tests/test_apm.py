# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Attitude Parameter Message (APM) Python bindings.
"""

import ccsds_ndm
import pytest

from ccsds_ndm import (
    AdmHeader,
    Apm,
    ApmData,
    ApmManeuverParameters,
    ApmMetadata,
    ApmSegment,
    QuaternionState,
)


class TestApm:
    """Tests for APM bindings."""

    def test_apm_metadata(self):
        meta = ApmMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            center_name="EARTH",
            time_system="UTC",
            # No ref_frame here
        )
        assert meta.object_name == "SAT1"
        assert meta.time_system == "UTC"

    def test_maneuver_parameters(self):
        man = ApmManeuverParameters(
            man_epoch_start="2023-01-01T00:00:00",
            man_duration=10.5,
            man_ref_frame="EME2000",
            man_tor_1=1.0,  # Float, not list
            man_tor_2=0.0,
            man_tor_3=0.0,
            man_delta_mass=None,
            comment=None,
        )
        assert man.man_duration == 10.5
        assert man.man_ref_frame == "EME2000"

    def test_apm_epoch_fields_require_calendar_or_ordinal_form(self):
        with pytest.raises(ValueError):
            ApmData(
                epoch="123.5",
                quaternion_state=[],
                euler_angle_state=None,
                angular_velocity=None,
                spin=None,
                inertia=None,
                maneuver_parameters=None,
                comment=None,
            )

        with pytest.raises(ValueError):
            ApmManeuverParameters(
                man_epoch_start="123.5",
                man_duration=10.0,
                man_ref_frame="EME2000",
                man_tor_1=1.0,
                man_tor_2=0.0,
                man_tor_3=0.0,
            )

    def test_apm_data(self):
        data = ApmData(
            epoch="2023-01-01T00:00:00",
            quaternion_state=[
                QuaternionState(
                    ref_frame_a="EME2000",
                    ref_frame_b="SC_BODY_1",
                    q1=0.0,
                    q2=0.0,
                    q3=0.0,
                    qc=1.0,
                    q1_dot=None,
                    q2_dot=None,
                    q3_dot=None,
                    qc_dot=None,
                    comment=None,
                )
            ],
            euler_angle_state=None,
            angular_velocity=None,
            spin=None,
            inertia=None,
            maneuver_parameters=None,
            comment=None,
        )
        assert data.quaternion_state[0].ref_frame_a == "EME2000"

    def _create_valid_apm(self):
        header = AdmHeader(
            classification="UNCLASSIFIED",
            creation_date="2023-01-01T00:00:00",
            originator="TEST",
            message_id="ID",
            comment=[],
        )
        meta = ApmMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            center_name="EARTH",
            time_system="UTC",
            comment=None,
        )
        q_state = QuaternionState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            q1=0.0,
            q2=0.0,
            q3=0.0,
            qc=1.0,
            q1_dot=None,
            q2_dot=None,
            q3_dot=None,
            qc_dot=None,
            comment=None,
        )
        data = ApmData(
            epoch="2023-01-01T00:00:00",
            quaternion_state=[q_state],
            euler_angle_state=None,
            angular_velocity=None,
            spin=None,
            inertia=None,
            maneuver_parameters=None,
            comment=None,
        )

        segment = ApmSegment(metadata=meta, data=data)
        return Apm(header=header, segment=segment)

    def test_construct_full_apm(self):
        apm = self._create_valid_apm()
        assert apm.header.originator == "TEST"
        assert apm.segment is not None

    def test_roundtrip_kvn(self):
        apm = self._create_valid_apm()
        kvn = apm.to_str(format="kvn")
        assert "CCSDS_APM_VERS" in kvn

        apm2 = Apm.from_str(kvn, format="kvn")
        assert apm2.header.originator == "TEST"

    def test_roundtrip_xml(self):
        apm = self._create_valid_apm()
        xml = apm.to_str(format="xml")
        assert "<apm" in xml

        apm2 = Apm.from_str(xml, format="xml")
        assert apm2.header.originator == "TEST"

    def test_file_io(self, tmp_path):
        apm = self._create_valid_apm()
        kvn_path = tmp_path / "test.apm"

        # Test to_file
        apm.to_file(str(kvn_path), "kvn")
        assert kvn_path.exists()

        # Test from_file
        apm2 = ccsds_ndm.from_file(str(kvn_path), format="kvn")
        assert apm2.header.originator == "TEST"

    def test_apm_setters(self):
        apm = self._create_valid_apm()

        # Top-level protocol identifier is read-only; version remains selectable.
        apm.version = "2.0"
        assert apm.id == "CCSDS_APM_VERS"
        assert apm.version == "2.0"

        # Segment setters
        segment = apm.segment
        meta = segment.metadata
        data = segment.data

        meta.object_name = "SAT2"
        meta.object_id = "2024-001A"
        meta.center_name = "EARTH"
        meta.time_system = "TAI"
        meta.comment = ["metadata comment"]
        assert meta.object_name == "SAT2"
        assert meta.object_id == "2024-001A"
        assert meta.center_name == "EARTH"
        assert meta.time_system == "TAI"
        assert meta.comment == ["metadata comment"]

        q_state = QuaternionState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            q1=0.1,
            q2=0.2,
            q3=0.3,
            qc=0.9,
            q1_dot=None,
            q2_dot=None,
            q3_dot=None,
            qc_dot=None,
            comment=None,
        )
        man = ApmManeuverParameters(
            man_epoch_start="2023-01-01T00:00:00",
            man_duration=1.0,
            man_ref_frame="EME2000",
            man_tor_1=1.0,
            man_tor_2=2.0,
            man_tor_3=3.0,
            man_delta_mass=None,
            comment=None,
        )

        # Maneuver setters
        man.man_epoch_start = "2023-01-02T00:00:00"
        man.man_duration = 2.0
        man.man_ref_frame = "ITRF"
        man.man_tor_x = 4.0
        man.man_tor_y = 5.0
        man.man_tor_z = 6.0
        man.man_delta_mass = -0.5
        man.comment = ["burn"]
        assert man.man_epoch_start == "2023-01-02T00:00:00"
        assert man.man_duration == 2.0
        assert man.man_ref_frame == "ITRF"
        assert man.man_tor_x == 4.0
        assert man.man_tor_y == 5.0
        assert man.man_tor_z == 6.0
        assert man.man_delta_mass == -0.5
        assert man.comment == ["burn"]

        # Data setters
        data.epoch = "2023-01-03T00:00:00"
        data.comment = ["data comment"]
        data.quaternion_state = [q_state]
        data.euler_angle_state = []
        data.angular_velocity = []
        data.spin = []
        data.inertia = []
        data.maneuver_parameters = [man]
        assert data.epoch == "2023-01-03T00:00:00"
        assert data.comment == ["data comment"]
        assert len(data.quaternion_state) == 1
        assert len(data.maneuver_parameters) == 1

        segment.metadata = meta
        segment.data = data
        apm.segment = segment
        assert apm.segment.metadata.object_name == "SAT2"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
