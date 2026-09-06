# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Attitude Comprehensive Message (ACM) Python bindings.
"""

import ccsds_ndm
import pytest

from ccsds_ndm import (
    Acm,
    AcmAttitudeDetermination,
    AcmAttitudeState,
    AcmCovarianceMatrix,
    AcmData,
    AcmManeuverParameters,
    AcmMetadata,
    AcmPhysicalDescription,
    AcmSegment,
    AcmSensor,
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

        meta.comment = ["metadata comment"]
        meta.catalog_name = "SATCAT"
        meta.object_designator = "12345"
        meta.originator_poc = "Jane Doe"
        meta.originator_position = "GNC Engineer"
        meta.originator_phone = "+12025550123"
        meta.originator_email = "jane@example.com"
        meta.originator_address = "123 Space Rd"
        meta.odm_msg_link = "ODM_MSG_12345.txt"
        meta.center_name = "EARTH"
        meta.time_system = "TAI"
        meta.epoch_tzero = "2023-01-01T01:00:00"
        meta.acm_data_elements = "ATT,COV,AD"
        meta.start_time = "2023-01-01T01:05:00"
        meta.stop_time = "2023-01-01T01:30:00"
        meta.taimutc_at_tzero = 37.0
        meta.next_leap_epoch = "2023-07-01T00:00:00"
        meta.next_leap_taimutc = 38.0

        assert meta.comment == ["metadata comment"]
        assert meta.catalog_name == "SATCAT"
        assert meta.object_designator == "12345"
        assert meta.originator_poc == "Jane Doe"
        assert meta.originator_position == "GNC Engineer"
        assert meta.originator_phone == "+12025550123"
        assert meta.originator_email == "jane@example.com"
        assert meta.originator_address == "123 Space Rd"
        assert meta.odm_msg_link == "ODM_MSG_12345.txt"
        assert meta.center_name == "EARTH"
        assert meta.time_system == "TAI"
        assert meta.epoch_tzero == "2023-01-01T01:00:00"
        assert meta.acm_data_elements == "ATT,COV,AD"
        assert meta.start_time == "2023-01-01T01:05:00"
        assert meta.stop_time == "2023-01-01T01:30:00"
        assert meta.taimutc_at_tzero == pytest.approx(37.0)
        assert meta.next_leap_epoch == "2023-07-01T00:00:00"
        assert meta.next_leap_taimutc == pytest.approx(38.0)

        meta.start_time = None
        meta.stop_time = None
        meta.taimutc_at_tzero = None
        meta.next_leap_epoch = None
        meta.next_leap_taimutc = None
        assert meta.start_time is None
        assert meta.stop_time is None
        assert meta.taimutc_at_tzero is None
        assert meta.next_leap_epoch is None
        assert meta.next_leap_taimutc is None

        with pytest.raises(ValueError):
            meta.epoch_tzero = "not-an-epoch"
        with pytest.raises(ValueError):
            meta.epoch_tzero = "123.5"
        with pytest.raises(ValueError):
            meta.next_leap_epoch = "123.5"

    def test_acm_attitude_state_invalid_att_type_raises(self):
        with pytest.raises(ValueError):
            AcmAttitudeState(
                ref_frame_a="EME2000",
                ref_frame_b="SC_BODY_1",
                att_type="NOT_IN_XSD_ENUM",
                att_lines=[[0.0, 0.0, 0.0, 1.0]],
                comment=[],
            )

    def test_acm_attitude_state_setters(self):
        state = AcmAttitudeState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            att_type="QUATERNION",
            att_lines=[[0.0, 0.0, 0.0, 1.0]],
            comment=[],
        )
        state.comment = ["att comment"]
        state.att_id = "ATT_1"
        state.att_prev_id = "ATT_0"
        state.att_basis = "PREDICTED"
        state.att_basis_id = "AD_1985"
        state.ref_frame_a = "J2000"
        state.ref_frame_b = "SC_BODY_2"
        state.number_states = 7
        state.att_type = "EULER_ANGLES"
        state.rate_type = "ANGVEL"
        state.euler_rot_seq = "XYZ"
        state.att_lines = [[1.0, 2.0, 3.0, 4.0], [4.0, 5.0, 6.0, 7.0]]

        assert state.comment == ["att comment"]
        assert state.att_id == "ATT_1"
        assert state.att_prev_id == "ATT_0"
        assert state.att_basis == "PREDICTED"
        assert state.att_basis_id == "AD_1985"
        assert state.ref_frame_a == "J2000"
        assert state.ref_frame_b == "SC_BODY_2"
        assert state.number_states == 7
        assert state.att_type == "EULER_ANGLES"
        assert state.rate_type == "ANGVEL"
        assert state.euler_rot_seq == "XYZ"
        assert state.att_lines == [[1.0, 2.0, 3.0, 4.0], [4.0, 5.0, 6.0, 7.0]]

        with pytest.raises(ValueError):
            state.att_type = "NOT_IN_XSD_ENUM"
        with pytest.raises(ValueError):
            state.att_basis = "NOT_IN_XSD_ENUM"
        with pytest.raises(ValueError):
            state.rate_type = "NOT_IN_XSD_ENUM"
        with pytest.raises(ValueError):
            state.euler_rot_seq = "BAD"

    def test_acm_covariance_matrix_invalid_cov_type_raises(self):
        with pytest.raises(ValueError):
            AcmCovarianceMatrix(
                cov_basis="SIMULATION",
                cov_ref_frame="EME2000",
                cov_type="NOT_IN_XSD_ENUM",
                cov_lines=[[1.0]],
                comment=[],
            )

    def test_acm_physical_description_setters(self):
        phys = AcmPhysicalDescription(comment=["initial"])
        assert phys.comment == ["initial"]

        phys.comment = ["updated"]
        phys.drag_coeff = 2.1
        phys.wet_mass = 750.0
        phys.dry_mass = 500.0
        phys.cp_ref_frame = "SC_BODY_1"
        phys.cp = [0.02, 0.01, 0.2]
        phys.inertia_ref_frame = "SC_BODY_1"
        phys.ixx = 1000.0
        phys.iyy = 800.0
        phys.izz = 400.0
        phys.ixy = 20.0
        phys.ixz = 40.0
        phys.iyz = 60.0

        assert phys.comment == ["updated"]
        assert phys.drag_coeff == pytest.approx(2.1)
        assert phys.wet_mass == pytest.approx(750.0)
        assert phys.dry_mass == pytest.approx(500.0)
        assert phys.cp_ref_frame == "SC_BODY_1"
        assert phys.cp == [0.02, 0.01, 0.2]
        assert phys.inertia_ref_frame == "SC_BODY_1"
        assert phys.ixx == pytest.approx(1000.0)
        assert phys.iyy == pytest.approx(800.0)
        assert phys.izz == pytest.approx(400.0)
        assert phys.ixy == pytest.approx(20.0)
        assert phys.ixz == pytest.approx(40.0)
        assert phys.iyz == pytest.approx(60.0)

        with pytest.raises(ValueError):
            phys.cp = [1.0, 2.0]

    def test_acm_covariance_matrix_setters(self):
        cov = AcmCovarianceMatrix(
            cov_basis="PREDICTED",
            cov_ref_frame="EME2000",
            cov_type="QUATERNION",
            cov_lines=[[1.0, 0.0, 0.0, 1.0]],
            comment=[],
        )
        cov.comment = ["cov comment"]
        cov.cov_basis = "DETERMINED_GND"
        cov.cov_ref_frame = "SC_BODY_1"
        cov.cov_type = "ANGLE"
        cov.cov_confidence = 80.0
        cov.cov_lines = [[1.0, 2.0], [3.0, 4.0]]

        assert cov.comment == ["cov comment"]
        assert cov.cov_basis == "DETERMINED_GND"
        assert cov.cov_ref_frame == "SC_BODY_1"
        assert cov.cov_type == "ANGLE"
        assert cov.cov_confidence == pytest.approx(80.0)
        assert cov.cov_lines == [[1.0, 2.0], [3.0, 4.0]]

        with pytest.raises(ValueError):
            cov.cov_type = "NOT_IN_XSD_ENUM"

    def test_acm_maneuver_parameters_setters(self):
        man = AcmManeuverParameters(man_id="MAN-1", comment=["initial"])
        man.comment = ["updated"]
        man.man_prev_id = "MAN-0"
        man.man_purpose = "ATT_ADJUST"
        man.man_begin_time = "100.0"
        man.man_end_time = "2.5e+2"
        man.man_duration = 60.0
        man.actuator_used = "RWA"
        man.target_momentum = [1.0, 2.0, 3.0]
        man.target_mom_frame = "SC_BODY_1"
        man.target_attitude = [0.0, 0.0, 0.0, 1.0]
        man.target_spinrate = 0.01

        assert man.comment == ["updated"]
        assert man.man_id == "MAN-1"
        assert man.man_prev_id == "MAN-0"
        assert man.man_purpose == "ATT_ADJUST"
        assert man.man_begin_time == "100.0"
        assert man.man_end_time == "2.5e+2"
        assert man.man_duration == pytest.approx(60.0)
        assert man.actuator_used == "RWA"
        assert man.target_momentum == [1.0, 2.0, 3.0]
        assert man.target_mom_frame == "SC_BODY_1"
        assert man.target_attitude == [0.0, 0.0, 0.0, 1.0]
        assert man.target_spinrate == pytest.approx(0.01)

        with pytest.raises(ValueError):
            man.target_momentum = [1.0, 2.0]
        with pytest.raises(ValueError):
            man.target_attitude = [0.0, 0.0, 1.0]
        with pytest.raises(ValueError):
            man.man_begin_time = "2023-01-01T00:00:00"
        with pytest.raises(ValueError):
            man.man_end_time = "NaN"

    def test_acm_attitude_determination_and_sensor_setters(self):
        sensor = AcmSensor(
            sensor_number=1,
            sensor_used="GYRO",
            sensor_noise_stddev=[0.01, 0.02],
            sensor_frequency=5.0,
            comment=["sensor comment"],
        )
        sensor.comment = ["updated sensor comment"]
        sensor.sensor_number = 2
        sensor.sensor_used = "AST"
        sensor.sensor_noise_stddev = [0.03]
        sensor.sensor_frequency = 2.0

        assert sensor.comment == ["updated sensor comment"]
        assert sensor.sensor_number == 2
        assert sensor.sensor_used == "AST"
        assert sensor.sensor_noise_stddev == [0.03]
        assert sensor.sensor_frequency == pytest.approx(2.0)

        ad = AcmAttitudeDetermination(ad_id="AD-1", comment=["ad comment"])
        ad.comment = ["updated ad comment"]
        ad.ad_prev_id = "AD-0"
        ad.ad_method = "EKF"
        ad.attitude_source = "OBC"
        ad.number_states = 7
        ad.attitude_states = "QUATERNION"
        ad.cov_type = "QUATERNION"
        ad.ad_epoch = "2023-01-01T00:00:00"
        ad.ref_frame_a = "J2000"
        ad.ref_frame_b = "SC_BODY_1"
        ad.attitude_type = "QUATERNION"
        ad.rate_states = "ANGVEL"
        ad.sigma_u = 3.7e-7
        ad.sigma_v = 1.3e-5
        ad.rate_process_noise_stddev = 5.1e-6
        ad.sensors = [sensor]

        assert ad.comment == ["updated ad comment"]
        assert ad.ad_id == "AD-1"
        assert ad.ad_prev_id == "AD-0"
        assert ad.ad_method == "EKF"
        assert ad.attitude_source == "OBC"
        assert ad.number_states == 7
        assert ad.attitude_states == "QUATERNION"
        assert ad.cov_type == "QUATERNION"
        assert ad.ad_epoch == "2023-01-01T00:00:00"
        assert ad.ref_frame_a == "J2000"
        assert ad.ref_frame_b == "SC_BODY_1"
        assert ad.attitude_type == "QUATERNION"
        assert ad.rate_states == "ANGVEL"
        assert ad.sigma_u == pytest.approx(3.7e-7)
        assert ad.sigma_v == pytest.approx(1.3e-5)
        assert ad.rate_process_noise_stddev == pytest.approx(5.1e-6)
        assert len(ad.sensors) == 1
        assert ad.sensors[0].sensor_number == 2

        with pytest.raises(ValueError):
            ad.attitude_states = "NOT_IN_XSD_ENUM"
        with pytest.raises(ValueError):
            ad.cov_type = "NOT_IN_XSD_ENUM"
        with pytest.raises(ValueError):
            ad.rate_states = "NOT_IN_XSD_ENUM"
        with pytest.raises(ValueError):
            ad.ad_epoch = "not-an-epoch"

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
            # Relative time followed by the four quaternion states.
            att_lines=[[0.0, 0.0, 0.0, 0.0, 1.0]],
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

    def test_live_physical_edits_are_revalidated_before_generation(self):
        acm = self._create_valid_acm()
        acm.segment.data.phys = AcmPhysicalDescription()
        acm.segment.data.phys.drag_coeff = float("nan")

        with pytest.raises(ccsds_ndm.NdmValidationError, match="DRAG_COEFF"):
            acm.to_str(format="xml")

    def test_file_io(self, tmp_path):
        acm = self._create_valid_acm()
        kvn_path = tmp_path / "test.acm"

        # Test to_file
        acm.to_file(str(kvn_path), "kvn")
        assert kvn_path.exists()

        # Test from_file
        acm2 = ccsds_ndm.from_file(str(kvn_path), format="kvn")
        assert acm2.header.originator == "TEST"

    def test_acm_setter_parity_for_nested_sections(self):
        acm = self._create_valid_acm()

        replacement_meta = AcmMetadata(
            object_name="SAT2",
            epoch_tzero="2023-02-01T00:00:00",
            time_system="UTC",
            international_designator="2023-002A",
        )
        replacement_data = AcmData(
            att=[], phys=None, cov=None, man=None, ad=None, user=None
        )
        replacement_segment = AcmSegment(
            metadata=replacement_meta, data=replacement_data
        )

        acm.segment = replacement_segment
        assert acm.segment.metadata.object_name == "SAT2"

        segment = acm.segment
        metadata = segment.metadata
        metadata.object_name = "SAT3"
        metadata.international_designator = None
        segment.metadata = metadata
        assert segment.metadata.object_name == "SAT3"
        assert segment.metadata.international_designator is None

        state = AcmAttitudeState(
            ref_frame_a="EME2000",
            ref_frame_b="SC_BODY_1",
            att_type="QUATERNION",
            att_lines=[[0.0, 0.0, 0.0, 1.0]],
            comment=[],
        )
        data = segment.data
        data.att = [state]
        assert len(data.att) == 1

        data.phys = AcmPhysicalDescription(comment=["shape"])
        assert data.phys is not None
        data.phys = None
        assert data.phys is None

        data.cov = [
            AcmCovarianceMatrix(
                cov_basis="PREDICTED",
                cov_ref_frame="EME2000",
                cov_type="QUATERNION",
                cov_lines=[[1.0, 0.0, 0.0, 1.0]],
                comment=[],
            )
        ]
        assert len(data.cov) == 1

        data.man = [AcmManeuverParameters(man_id="MAN-1", comment=[])]
        assert len(data.man) == 1

        data.ad = AcmAttitudeDetermination(ad_id="AD-1", comment=[])
        assert data.ad is not None
        data.ad = None
        assert data.ad is None

        segment.data = data
        acm.segment = segment
        assert len(acm.segment.data.att) == 1


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
