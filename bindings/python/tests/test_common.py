# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for the shared header classes (OdmHeader, AdmHeader).
"""

import pytest

from ccsds_ndm import AdmHeader, OdmHeader


@pytest.mark.parametrize("header_type", [OdmHeader, AdmHeader])
def test_creation_date_requires_absolute_epoch(header_type):
    with pytest.raises(ValueError):
        header_type("123.5", "TEST")

    header = header_type("2002-204T15:56:23Z", "TEST")
    assert header.creation_date == "2002-204T15:56:23Z"
    with pytest.raises(ValueError):
        header.creation_date = "123.5"


def test_numpy_inputs_name_their_expected_dimensions():
    # A 1-D array used to fail inside NumPy's typed extraction with
    # "'ndarray' object is not an instance of 'ndarray'".
    import numpy as np
    from ccsds_ndm import AemData, CdmCovarianceMatrix, OemData

    epochs = ["2023-01-01T00:00:00"]
    oem = OemData.from_numpy(
        state_vector_epochs=epochs, state_vector_numpy=np.zeros((1, 6))
    )
    for call in (
        lambda: OemData.from_numpy(
            state_vector_epochs=epochs, state_vector_numpy=np.zeros(6)
        ),
        lambda: setattr(oem, "state_vector_numpy", np.zeros(6)),
        lambda: AemData.from_numpy(epochs, np.zeros(4), "QUATERNION"),
        lambda: CdmCovarianceMatrix.from_numpy(np.zeros(36)),
    ):
        with pytest.raises(ValueError, match=r"must be a 2-D array; got shape"):
            call()
