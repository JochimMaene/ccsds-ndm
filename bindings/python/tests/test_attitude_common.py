# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for shared attitude state classes (QuaternionState, EulerAngleState, etc.).
"""

import pytest

from ccsds_ndm import (
    AngVelState,
    EulerAngleState,
    InertiaState,
    QuaternionState,
    SpinState,
)


class TestAttitudeStates:
    """Tests for shared attitude state classes defined in attitude.rs."""

    def test_quaternion_state(self):
        qs = QuaternionState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            q1=0.1,
            q2=0.2,
            q3=0.3,
            qc=0.4,
            q1_dot=0.01,
            q2_dot=0.02,
            q3_dot=0.03,
            qc_dot=0.04,
            comment=["Test Quaternion"],
        )
        assert qs.ref_frame_a == "EME2000"
        assert qs.q1 == 0.1
        # Setters
        qs.q1 = 0.9
        assert qs.q1 == 0.9
        assert qs.quaternion_dot == [0.01, 0.02, 0.03, 0.04]
        qs.quaternion_dot = [0.11, 0.12, 0.13, 0.14]
        assert qs.quaternion_dot == [0.11, 0.12, 0.13, 0.14]
        qs.quaternion_dot = None
        assert qs.quaternion_dot is None
        with pytest.raises(ValueError):
            qs.quaternion_dot = [0.1, 0.2, 0.3]
        assert qs.comment == ["Test Quaternion"]

    def test_euler_angle_state(self):
        es = EulerAngleState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            euler_rot_seq="ZXZ",
            angle_1=10.0,
            angle_2=20.0,
            angle_3=30.0,
            angle_1_dot=1.0,
            angle_2_dot=2.0,
            angle_3_dot=3.0,
            comment=[],
        )
        assert es.euler_rot_seq == "ZXZ"
        assert es.angle_1 == 10.0
        assert es.angle_1_dot == 1.0
        es.angle_1 = 11.0
        es.angle_2 = 21.0
        es.angle_3 = 31.0
        es.angle_1_dot = None
        es.euler_rot_seq = "ZYX"
        es.comment = ["Updated"]
        assert es.angle_1 == 11.0
        assert es.angle_2 == 21.0
        assert es.angle_3 == 31.0
        assert es.angle_1_dot is None
        assert es.euler_rot_seq == "ZYX"
        assert es.comment == ["Updated"]

    def test_ang_vel_state(self):
        av = AngVelState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            angvel_frame="SC_BODY_1",
            angvel_x=0.1,
            angvel_y=0.2,
            angvel_z=0.3,
            comment=[],
        )
        assert av.angvel_frame == "SC_BODY_1"
        assert av.angvel_x == 0.1
        av.angvel_frame = "EME2000"
        av.angvel_x = 1.1
        av.angvel_y = 1.2
        av.angvel_z = 1.3
        av.comment = ["Updated"]
        assert av.angvel_frame == "EME2000"
        assert av.angvel_x == 1.1
        assert av.angvel_y == 1.2
        assert av.angvel_z == 1.3
        assert av.comment == ["Updated"]

    def test_spin_state(self):
        ss = SpinState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            spin_alpha=10.0,
            spin_delta=20.0,
            spin_angle=30.0,
            spin_angle_vel=40.0,
            nutation=1.0,
            nutation_per=2.0,
            nutation_phase=3.0,
            momentum_alpha=4.0,
            momentum_delta=5.0,
            nutation_vel=6.0,
            comment=[],
        )
        assert ss.spin_alpha == 10.0
        assert ss.nutation == 1.0
        ss.spin_alpha = 11.0
        ss.spin_delta = 21.0
        ss.spin_angle = 31.0
        ss.spin_angle_vel = 41.0
        ss.nutation = None
        ss.nutation_per = 3.0
        ss.nutation_phase = None
        ss.momentum_alpha = 4.5
        ss.momentum_delta = 5.5
        ss.nutation_vel = None
        ss.comment = ["Updated"]
        assert ss.spin_alpha == 11.0
        assert ss.spin_delta == 21.0
        assert ss.spin_angle == 31.0
        assert ss.spin_angle_vel == 41.0
        assert ss.nutation is None
        assert ss.nutation_per == 3.0
        assert ss.nutation_phase is None
        assert ss.momentum_alpha == 4.5
        assert ss.momentum_delta == 5.5
        assert ss.nutation_vel is None
        assert ss.comment == ["Updated"]

    def test_inertia_state(self):
        is_ = InertiaState(
            inertia_ref_frame="SC_BODY_1",
            ixx=1.0,
            iyy=2.0,
            izz=3.0,
            ixy=0.1,
            ixz=0.2,
            iyz=0.3,
            comment=[],
        )
        assert is_.inertia_ref_frame == "SC_BODY_1"
        assert is_.ixx == 1.0
        is_.inertia_ref_frame = "EME2000"
        is_.ixx = 10.0
        is_.iyy = 20.0
        is_.izz = 30.0
        is_.ixy = 0.4
        is_.ixz = 0.5
        is_.iyz = 0.6
        is_.comment = ["Updated"]
        assert is_.inertia_ref_frame == "EME2000"
        assert is_.ixx == 10.0
        assert is_.iyy == 20.0
        assert is_.izz == 30.0
        assert is_.ixy == 0.4
        assert is_.ixz == 0.5
        assert is_.iyz == 0.6
        assert is_.comment == ["Updated"]


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
