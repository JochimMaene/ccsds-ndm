# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

from pathlib import Path

import ccsds_ndm

ROOT = Path(__file__).resolve().parents[3]


def test_opm_and_apm_maneuver_types_have_unique_public_names():
    opm_maneuver = ccsds_ndm.OpmManeuverParameters(
        "2021-06-03T09:00:34.1",
        132.6,
        -18.418,
        "EME2000",
        -0.023257,
        0.0168316,
        -0.00893444,
    )
    apm_maneuver = ccsds_ndm.ApmManeuverParameters(
        man_epoch_start="2023-01-01T00:00:00",
        man_duration=10.5,
        man_ref_frame="EME2000",
        man_tor_1=1.0,
        man_tor_2=0.0,
        man_tor_3=0.0,
    )

    assert type(opm_maneuver) is not type(apm_maneuver)
    assert not hasattr(ccsds_ndm, "ManeuverParameters")

    opm = ccsds_ndm.from_file(str(ROOT / "ccsds-ndm/data/kvn/opm_g2.kvn"))
    opm.segment.data.maneuver_parameters = [opm_maneuver]
    assert (
        type(opm.segment.data.maneuver_parameters[0]) is ccsds_ndm.OpmManeuverParameters
    )

    apm = ccsds_ndm.from_file(str(ROOT / "ccsds-ndm/data/kvn/apm_g1.kvn"))
    apm.segment.data.maneuver_parameters = [apm_maneuver]
    assert (
        type(apm.segment.data.maneuver_parameters[0]) is ccsds_ndm.ApmManeuverParameters
    )
