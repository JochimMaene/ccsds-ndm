# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Orbit Comprehensive Message (OCM) Python bindings.
"""

import ccsds_ndm
import pytest

from ccsds_ndm import (
    CovLine,
    ManLine,
    NdmValidationError,
    Ocm,
    OcmCovarianceMatrix,
    OcmData,
    OcmManeuverParameters,
    OcmMetadata,
    OcmOdParameters,
    OcmPhysicalDescription,
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

    def test_history_time_tags_use_one_epoch_branch(self):
        ocm = self._create_valid_ocm()
        data = ocm.segment.data
        data.traj = [
            OcmTrajState(
                center_name="EARTH",
                traj_ref_frame="GCRF",
                traj_type="CARTPV",
                traj_lines=[
                    TrajLine(epoch="2023-01-01T00:00:00", values=[1.0]),
                    TrajLine(epoch="1.0", values=[2.0]),
                ],
            )
        ]
        segment = ocm.segment
        segment.data = data
        ocm.segment = segment

        with pytest.raises(NdmValidationError):
            ocm.validate()

    def test_metadata_reference_epochs_require_calendar_form(self):
        with pytest.raises(ValueError):
            OcmMetadata(epoch_tzero="123.5")

        metadata = OcmMetadata(epoch_tzero="2023-01-01T00:00:00")
        for field in (
            "epoch_tzero",
            "previous_message_epoch",
            "next_message_epoch",
            "next_leap_epoch",
        ):
            with pytest.raises(ValueError):
                setattr(metadata, field, "123.5")

    def test_frame_reference_epochs_require_calendar_form(self):
        with pytest.raises(ValueError):
            OcmTrajState(
                center_name="EARTH",
                traj_ref_frame="GCRF",
                traj_type="CARTPV",
                traj_lines=[TrajLine(epoch="123.5", values=[1.0])],
                traj_frame_epoch="123.5",
            )

        with pytest.raises(ValueError):
            OcmPhysicalDescription(oeb_parent_frame_epoch="123.5")

        with pytest.raises(ValueError):
            OcmCovarianceMatrix(
                cov_ref_frame="GCRF",
                cov_type="CARTPV",
                cov_ordering="LTM",
                cov_lines=[CovLine(epoch="123.5", values=[1.0])],
                cov_frame_epoch="123.5",
            )

        with pytest.raises(ValueError):
            OcmManeuverParameters(
                man_id="MAN-1",
                man_device_id="THR-1",
                man_composition="TIME_ABSOLUTE",
                man_ref_frame="GCRF",
                man_lines=[ManLine(epoch="123.5", values=["1.0"])],
                man_frame_epoch="123.5",
            )

        with pytest.raises(ValueError):
            OcmTrajState(
                center_name="EARTH",
                traj_ref_frame="GCRF",
                traj_type="CARTPV",
                traj_lines=[TrajLine(epoch="123.5", values=[1.0])],
                useable_start_time="123.5",
            )

        with pytest.raises(ValueError):
            OcmTrajState(
                center_name="EARTH",
                traj_ref_frame="GCRF",
                traj_type="CARTPV",
                traj_lines=[TrajLine(epoch="123.5", values=[1.0])],
                useable_stop_time="123.5",
            )

    def test_file_io(self, tmp_path):
        ocm = self._create_valid_ocm()
        path = tmp_path / "test.ocm"

        ocm.to_file(str(path), "kvn")
        assert path.exists()

        ocm2 = ccsds_ndm.from_file(str(path), format="kvn")
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
            man_composition="TIME_ABSOLUTE",
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

    def test_ocm_od_parameters_days_since_setters(self):
        od = OcmOdParameters(
            od_id="OD-1",
            od_method="LEAST_SQUARES",
            od_epoch="2023-01-01T00:00:00",
        )

        assert od.days_since_first_obs is None
        assert od.days_since_last_obs is None

        od.days_since_first_obs = 2.5
        od.days_since_last_obs = -0.75
        assert od.days_since_first_obs == pytest.approx(2.5)
        assert od.days_since_last_obs == pytest.approx(-0.75)

        od.days_since_first_obs = None
        od.days_since_last_obs = None
        assert od.days_since_first_obs is None
        assert od.days_since_last_obs is None

    def test_mutated_physical_and_od_values_are_revalidated(self):
        ocm = self._create_valid_ocm()
        ocm.segment.data.phys = OcmPhysicalDescription(wet_mass=100.0)
        phys = ocm.segment.data.phys
        phys.wet_mass = -1.0

        with pytest.raises(NdmValidationError, match="WET_MASS"):
            ocm.validate()
        with pytest.raises(NdmValidationError):
            ocm.to_str(format="kvn")
        with pytest.raises(NdmValidationError):
            ocm.to_str(format="xml")

        phys.wet_mass = 100.0
        ocm.validate()

        ocm.segment.data.od = OcmOdParameters(
            od_id="OD-1",
            od_method="LEAST_SQUARES",
            od_epoch="2023-01-01T00:00:00",
        )
        ocm.segment.data.od.weighted_rms = float("nan")
        with pytest.raises(NdmValidationError, match="WEIGHTED_RMS"):
            ocm.validate()

    def test_mutated_traj_line_value_is_revalidated(self):
        ocm = self._create_valid_ocm()
        traj = ocm.segment.data.traj[0]
        traj.traj_lines = [
            TrajLine(
                epoch="2023-01-01T00:00:00",
                values=[7000.0, 0.0, float("inf"), 0.0, 7.5, 0.0],
            )
        ]
        ocm.segment.data.traj = [traj]

        with pytest.raises(NdmValidationError, match="trajLine"):
            ocm.validate()

    def test_enum_constructor_arguments_are_not_discarded(self):
        """The constructor accepted `traj_basis` and `orb_revnum_basis` and silently dropped
        them, on the false premise that the enums lacked `FromStr`."""
        traj = OcmTrajState(
            center_name="EARTH",
            traj_ref_frame="EME2000",
            traj_type="CARTPV",
            traj_basis="PREDICTED",
            orb_revnum_basis="1",
            traj_lines=[TrajLine(epoch="2023-01-01T00:00:00", values=[1.0])],
        )
        assert traj.traj_basis == "PREDICTED"
        assert traj.orb_revnum_basis == "1"

        with pytest.raises(ValueError):
            OcmTrajState(
                center_name="EARTH",
                traj_ref_frame="EME2000",
                traj_type="CARTPV",
                traj_basis="NOT_A_BASIS",
                traj_lines=[TrajLine(epoch="2023-01-01T00:00:00", values=[1.0])],
            )

    def test_enum_getters_return_the_wire_spelling(self):
        """Getters used Rust's `Debug` spelling, so `orb_revnum_basis` read back as `'One'` and
        assigning it straight back raised. A property's own value must be assignable to it."""
        ocm = self._create_valid_ocm()
        traj = ocm.segment.data.traj[0]
        traj.traj_basis = "DETERMINED"
        traj.orb_revnum_basis = "0"

        assert traj.traj_basis == "DETERMINED"
        assert traj.orb_revnum_basis == "0"

        for name in ("traj_basis", "orb_revnum_basis"):
            value = getattr(traj, name)
            setattr(traj, name, value)  # must not raise
            assert getattr(traj, name) == value

    def test_traj_line_epoch_is_validated_without_changing_string_api(self):
        line = TrajLine(epoch="2023-001T00:00:00", values=[1.0])
        assert line.epoch == "2023-001T00:00:00"

        line.epoch = "123.5"
        assert line.epoch == "123.5"

        with pytest.raises(ValueError):
            TrajLine(epoch="not-an-epoch", values=[1.0])
        with pytest.raises(ValueError):
            line.epoch = "not-an-epoch"

    def test_cov_line_epoch_is_validated_without_changing_string_api(self):
        line = CovLine(epoch="2023-001T00:00:00", values=[1.0])
        assert line.epoch == "2023-001T00:00:00"

        line.epoch = "123.5"
        assert line.epoch == "123.5"

        with pytest.raises(ValueError):
            CovLine(epoch="not-an-epoch", values=[1.0])
        with pytest.raises(ValueError):
            line.epoch = "not-an-epoch"

    def test_man_line_epoch_is_validated_without_changing_string_api(self):
        line = ManLine(epoch="2023-001T00:00:00", values=["1.0"])
        assert line.epoch == "2023-001T00:00:00"

        line.epoch = "123.5"
        assert line.epoch == "123.5"

        with pytest.raises(ValueError):
            ManLine(epoch="not-an-epoch", values=["1.0"])
        with pytest.raises(ValueError):
            line.epoch = "not-an-epoch"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
