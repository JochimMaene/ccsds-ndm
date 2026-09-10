# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""Benchmark the NumPy bulk constructor exposed by the Python bindings.

`OemData.from_numpy` is the recommended way to build a large ephemeris from Python, so it is
the one binding path worth tracking. The epoch list and the array are built outside the measured
call: they are the caller's own data, and including them would measure NumPy and CPython rather
than the PyO3 conversion.
"""

import numpy as np
from ccsds_ndm import OemData

RECORDS = 10_000


def test_oem_data_from_numpy(benchmark) -> None:
    epochs = [
        f"2023-01-01T{i // 3600:02}:{i // 60 % 60:02}:{i % 60:02}"
        for i in range(RECORDS)
    ]
    states = np.tile(
        np.array([7000.0, 0.0, 0.0, 0.0, 7.5, 0.0], dtype=float),
        (RECORDS, 1),
    )

    data = benchmark(
        lambda: OemData.from_numpy(
            state_vector_epochs=epochs,
            state_vector_numpy=states,
        )
    )

    assert data.state_vector_numpy.shape == (RECORDS, 6)
