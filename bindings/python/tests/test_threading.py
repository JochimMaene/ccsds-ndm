# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""The Rust core runs with the GIL released, so other Python threads keep running."""

import threading
from datetime import datetime, timedelta

import ccsds_ndm
import numpy as np
from ccsds_ndm import Oem, OdmHeader, OemData, OemMetadata, OemSegment


def _large_oem_kvn(records: int) -> str:
    start = datetime(2023, 1, 1)
    epochs = [(start + timedelta(seconds=i)).isoformat() for i in range(records)]
    states = np.tile([7000.0, 0.0, 0.0, 0.0, 7.5, 0.0], (records, 1))
    metadata = OemMetadata(
        "SAT1",
        "2023-001A",
        epochs[0],
        epochs[-1],
        center_name="EARTH",
        ref_frame="EME2000",
        time_system="UTC",
    )
    data = OemData.from_numpy(state_vector_epochs=epochs, state_vector_numpy=states)
    header = OdmHeader("2023-01-01T00:00:00", "TEST", None, None, None)
    return Oem(header, [OemSegment(metadata, data)]).to_str("kvn")


def _progress_during(call) -> int:
    """Count pure-Python loop iterations another thread completes while `call` runs."""
    counter = [0]
    stop = threading.Event()

    def spin() -> None:
        while not stop.is_set():
            counter[0] += 1

    thread = threading.Thread(target=spin)
    thread.start()
    try:
        before = counter[0]
        call()
        return counter[0] - before
    finally:
        stop.set()
        thread.join()


def test_other_threads_progress_during_core_parsing_and_generation():
    kvn = _large_oem_kvn(20_000)
    message = ccsds_ndm.from_str(kvn, format="kvn")
    # While the GIL is held, the spinning thread cannot run at all during the call; with it
    # released it completes a large number of iterations.
    assert _progress_during(lambda: ccsds_ndm.from_str(kvn, format="kvn")) > 1000
    assert _progress_during(lambda: ccsds_ndm.convert(kvn, "xml")) > 1000
    assert _progress_during(lambda: Oem.from_str(kvn, format="kvn")) > 1000
    assert isinstance(message, Oem)
