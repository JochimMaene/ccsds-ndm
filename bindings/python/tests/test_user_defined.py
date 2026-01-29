# SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
#
# SPDX-License-Identifier: MPL-2.0

import ccsds_ndm

def test_acm_user_defined():
    kvn = """CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
USER_DEFINED_FOO = BAR
USER_STOP
"""
    acm = ccsds_ndm.from_str(kvn)
    user = acm.segment.data.user
    assert user is not None
    assert user.user_defined["FOO"] == "BAR"
    
    # Roundtrip check
    kvn_out = acm.to_str("kvn")
    assert "USER_START" in kvn_out
    assert "USER_DEFINED_FOO" in kvn_out
    assert "BAR" in kvn_out
    assert "USER_STOP" in kvn_out

def test_ocm_user_defined():
    kvn = """CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
USER_DEFINED_BAZ = QUX
USER_STOP
"""
    ocm = ccsds_ndm.from_str(kvn)
    user = ocm.segment.data.user
    assert user is not None
    assert user.user_defined["BAZ"] == "QUX"

def test_opm_user_defined():
    kvn = """CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 1000
Y = 2000
Z = 3000
X_DOT = 1
Y_DOT = 2
Z_DOT = 3
USER_DEFINED_PARAM = VALUE
"""
    opm = ccsds_ndm.from_str(kvn)
    ud = opm.segment.data.user_defined_parameters
    assert ud is not None
    assert ud.user_defined["PARAM"] == "VALUE"

def test_rdm_user_defined():
    kvn = """CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = 1
OBJECT_NAME = SAT1
INTERNATIONAL_DESIGNATOR = 1
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 100 [km]
USER_DEFINED_TEST = RDM_VALUE
"""
    rdm = ccsds_ndm.from_str(kvn)
    ud = rdm.segment.data.user_defined_parameters
    assert ud is not None
    assert ud.user_defined["TEST"] == "RDM_VALUE"
