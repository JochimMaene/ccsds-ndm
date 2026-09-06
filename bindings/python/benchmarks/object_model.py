"""Reproducible timing and peak-RSS baseline for the mutable Python object model."""

from __future__ import annotations

import argparse
import gc
import json
import resource
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

from ccsds_ndm import OdmHeader, Oem, OemData, OemMetadata, OemSegment, StateVectorAcc


def timed(operation, iterations: int, warmup: int) -> float:
    """Median of `iterations` samples, after `warmup` untimed calls.

    The warm-up exists because the first call through a PyO3 boundary pays one-off costs — import
    resolution, allocator arena growth, and CPU frequency ramp — that are not part of steady-state
    behaviour and would otherwise land in the sample set.
    """
    for _ in range(warmup):
        operation()

    samples = []
    gc.disable()
    try:
        for _ in range(iterations):
            started = time.perf_counter_ns()
            operation()
            samples.append(time.perf_counter_ns() - started)
    finally:
        gc.enable()
    return statistics.median(samples) / 1_000_000


def caller_arrays(count: int):
    """Caller-owned epoch strings and six-column state values with a valid timeline."""
    import datetime

    base = datetime.datetime(2023, 1, 1, tzinfo=datetime.timezone.utc)
    epochs = [
        (base + datetime.timedelta(seconds=60 * i)).strftime("%Y-%m-%dT%H:%M:%S")
        for i in range(count)
    ]
    values = [(7000.0 + i * 0.1, float(i), 0.0, 0.0, 7.5, 0.0) for i in range(count)]
    return epochs, values


def build_oem_from_records(epochs, values):
    header = OdmHeader("2023-01-01T00:00:00", "BENCH")
    meta = OemMetadata(
        "BENCH",
        "2023-001A",
        epochs[0],
        epochs[-1],
        center_name="EARTH",
        ref_frame="EME2000",
        time_system="UTC",
    )
    states = [
        StateVectorAcc(
            epoch=epoch, x=v[0], y=v[1], z=v[2], x_dot=v[3], y_dot=v[4], z_dot=v[5]
        )
        for epoch, v in zip(epochs, values)
    ]
    return Oem(header, [OemSegment(meta, OemData(state_vectors=states, comments=None))])


def build_oem_from_numpy(epochs, values):
    import numpy as np

    header = OdmHeader("2023-01-01T00:00:00", "BENCH")
    meta = OemMetadata(
        "BENCH",
        "2023-001A",
        epochs[0],
        epochs[-1],
        center_name="EARTH",
        ref_frame="EME2000",
        time_system="UTC",
    )
    array = np.array(values, dtype=float)
    data = OemData.from_numpy(
        state_vector_epochs=list(epochs),
        state_vector_numpy=array,
        comments=[],
    )
    return Oem(header, [OemSegment(meta, data)])


def worker(document: Path, operation: str, iterations: int, warmup: int) -> None:
    notation = "kvn" if operation.endswith("_kvn") else "xml"
    source = document.read_text()
    record_count = int(document.stem.split("-")[-1])
    needs_message = not (
        operation.startswith("parse") or operation.startswith("construct")
    )
    message = None if not needs_message else Oem.from_str(source, notation)

    if operation == "parse":

        def run():
            Oem.from_str(source, "xml")

    elif operation == "parse_kvn":

        def run():
            Oem.from_str(source, "kvn")

    elif operation == "generate_kvn":

        def run():
            message.to_str("kvn")

    elif operation == "validate":
        run = message.validate
    elif operation == "generate_xml":

        def run():
            message.to_str("xml")

    elif operation == "retained_edit":
        state = message.segments[0].data.state_vector[0]
        original = state.x

        def run():
            state.x = original + 1.0
            state.x = original

    elif operation == "edit_and_validate":
        state = message.segments[0].data.state_vector[0]
        original = state.x

        def run():
            state.x = original + 1.0
            message.validate()
            state.x = original

    elif operation == "construct_records":
        epochs, values = caller_arrays(record_count)

        def run():
            build_oem_from_records(epochs, values)

    elif operation == "construct_numpy":
        epochs, values = caller_arrays(record_count)

        def run():
            build_oem_from_numpy(epochs, values)

    elif operation == "construct_to_kvn":
        epochs, values = caller_arrays(record_count)

        def run():
            build_oem_from_records(epochs, values).to_str("kvn")

    elif operation == "construct_numpy_to_kvn":
        epochs, values = caller_arrays(record_count)

        def run():
            build_oem_from_numpy(epochs, values).to_str("kvn")

    elif operation == "construct_to_xml":
        epochs, values = caller_arrays(record_count)

        def run():
            build_oem_from_records(epochs, values).to_str("xml")

    elif operation == "construct_numpy_to_xml":
        epochs, values = caller_arrays(record_count)

        def run():
            build_oem_from_numpy(epochs, values).to_str("xml")

    elif operation == "numeric_access":
        import numpy as np

        def run():
            data = message.segments[0].data
            array = data.state_vector_numpy
            float(np.sum(array[:, 0]))

    else:
        raise ValueError(operation)

    result = {
        "operation": operation,
        "median_ms": timed(run, iterations, warmup),
        "peak_rss_kib": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
    }
    print(json.dumps(result))


def make_document(source: str, records: int, destination: Path, notation: str) -> None:
    message = Oem.from_str(source, "xml")
    data = message.segments[0].data
    state = data.state_vector[0]
    # Repeating one wrapper keeps *document construction* cheap. It does not carry into the
    # measurements: each worker parses the serialized document and builds its own distinct object
    # graph, so parse, generate, and validate all see `records` independent records.
    data.state_vector = [state] * records
    data.covariance_matrix = []
    destination.write_text(message.to_str(notation))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--records", type=int, nargs="+", default=[10, 1_000, 100_000])
    parser.add_argument("--iterations", type=int, default=10)
    parser.add_argument("--warmup", type=int, default=3)
    parser.add_argument("--worker", action="store_true")
    parser.add_argument("--document", type=Path)
    parser.add_argument("--operation")
    args = parser.parse_args()

    if args.worker:
        worker(args.document, args.operation, args.iterations, args.warmup)
        return

    repository = Path(__file__).resolve().parents[3]
    source = (repository / "ccsds-ndm/data/xml/oem_g14.xml").read_text()
    # Notation per operation, so both wire formats are measured rather than XML alone.
    operations = [
        ("parse", "xml"),
        ("generate_xml", "xml"),
        ("parse_kvn", "kvn"),
        ("generate_kvn", "kvn"),
        ("validate", "xml"),
        ("retained_edit", "xml"),
        ("edit_and_validate", "xml"),
        ("construct_records", "xml"),
        ("construct_numpy", "xml"),
        ("construct_to_kvn", "xml"),
        ("construct_numpy_to_kvn", "xml"),
        ("construct_to_xml", "xml"),
        ("construct_numpy_to_xml", "xml"),
        ("numeric_access", "xml"),
    ]
    results = []
    with tempfile.TemporaryDirectory() as directory:
        for records in args.records:
            documents = {}
            for notation in ("xml", "kvn"):
                document = Path(directory) / f"oem-{records}.{notation}"
                make_document(source, records, document, notation)
                documents[notation] = document
            for operation, notation in operations:
                document = documents[notation]
                command = [
                    sys.executable,
                    __file__,
                    "--worker",
                    "--document",
                    str(document),
                    "--operation",
                    operation,
                    "--iterations",
                    str(args.iterations),
                    "--warmup",
                    str(args.warmup),
                ]
                completed = subprocess.run(
                    command, check=True, capture_output=True, text=True
                )
                result = json.loads(completed.stdout)
                result["records"] = records
                result["document_bytes"] = document.stat().st_size
                results.append(result)

    print(json.dumps({"python": sys.version, "results": results}, indent=2))


if __name__ == "__main__":
    main()
