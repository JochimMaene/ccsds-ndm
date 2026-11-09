# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Orbit Parameter Message (OPM) Python bindings.
"""

import pytest

from ccsds_ndm import (
    KeplerianElements,
    OdmHeader,
    Opm,
    OpmData,
    OpmMetadata,
    OpmSegment,
    StateVector,
)
from ccsds_ndm import NdmEpochError


class TestOpm:
    """Tests for OPM bindings."""

    def _create_valid_opm(self):
        header = OdmHeader("2023-01-01T00:00:00", "TEST", "UNCLASSIFIED", "ID", [])

        meta = OpmMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            center_name="EARTH",
            ref_frame="GCRF",
            time_system="UTC",
        )

        # StateVector signature needs comment=None explicitly
        state = StateVector(
            epoch="2023-01-01T00:00:00",
            x=7000.0,
            y=0.0,
            z=0.0,
            x_dot=0.0,
            y_dot=7.5,
            z_dot=0.0,
            comment=None,
        )

        # KeplerianElements signature:
        kepler = KeplerianElements(
            semi_major_axis=7000.0,
            eccentricity=0.001,
            inclination=98.0,
            ra_of_asc_node=10.0,
            arg_of_pericenter=20.0,
            gm=398600.44,
            true_anomaly=0.0,
            mean_anomaly=None,
        )

        data = OpmData(state_vector=state, comment=[])
        data.keplerian_elements = kepler

        seg = OpmSegment(metadata=meta, data=data)

        return Opm(header=header, segment=seg)

    @pytest.mark.parametrize("format", ["kvn", "xml"])
    def test_constructed_message_roundtrips(self, format):
        message = self._create_valid_opm()
        parsed = Opm.from_str(message.to_str(format), format=format)
        assert parsed.header.originator == "TEST"
        assert parsed.segment.data.state_vector.x == 7000.0

    def test_epoch_fields_require_calendar_form(self):
        with pytest.raises(NdmEpochError):
            OpmMetadata(
                object_name="SAT1",
                object_id="2023-001A",
                center_name="EARTH",
                ref_frame="GCRF",
                time_system="UTC",
                ref_frame_epoch="12345.5",
            )

        with pytest.raises(NdmEpochError):
            StateVector(
                epoch="12345.5",
                x=7000.0,
                y=0.0,
                z=0.0,
                x_dot=0.0,
                y_dot=7.5,
                z_dot=0.0,
                comment=None,
            )
