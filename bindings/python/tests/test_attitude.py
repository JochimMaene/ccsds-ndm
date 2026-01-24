# SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
#
# SPDX-License-Identifier: MPL-2.0

import ccsds_ndm


def test_aem_parsing():
    kvn = """CCSDS_AEM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST

META_START
OBJECT_NAME = SAT1
OBJECT_ID = 2023-001A
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
ATTITUDE_TYPE = QUATERNION
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
META_STOP

DATA_START
2023-01-01T00:00:00 0.5 0.5 0.5 0.5
DATA_STOP
"""
    aem = ccsds_ndm.from_str(kvn)
    assert isinstance(aem, ccsds_ndm.Aem)
    assert aem.header.originator == "TEST"
    assert aem.segments[0].metadata.object_name == "SAT1"
    assert len(aem.segments[0].data.attitude_states) == 1


def test_apm_parsing():
    kvn = """CCSDS_APM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST

META_START
OBJECT_NAME = SAT1
OBJECT_ID = 2023-001A
TIME_SYSTEM = UTC
META_STOP

QUAT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
Q1 = 0.5
Q2 = 0.5
Q3 = 0.5
QC = 0.5
QUAT_STOP
"""
    apm = ccsds_ndm.from_str(kvn)
    assert isinstance(apm, ccsds_ndm.Apm)
    assert apm.header.originator == "TEST"
    assert apm.segment.metadata.object_name == "SAT1"
    assert apm.segment.data.quaternion_state is not None


def test_acm_parsing():
    kvn = """CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST

META_START
OBJECT_NAME = SAT1
INTERNATIONAL_DESIGNATOR = 2023-001A
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP

ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 1
ATT_TYPE = QUATERNION
0.5 0.5 0.5 0.5
ATT_STOP
"""
    acm = ccsds_ndm.from_str(kvn)
    assert isinstance(acm, ccsds_ndm.Acm)
    assert acm.header.originator == "TEST"
    assert acm.segment.metadata.object_name == "SAT1"
    assert len(acm.segment.data.att) == 1
