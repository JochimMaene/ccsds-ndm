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

from ccsds_ndm import Oem


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


def worker(document: Path, operation: str, iterations: int, warmup: int) -> None:
    notation = "kvn" if operation.endswith("_kvn") else "xml"
    source = document.read_text()
    message = None if operation.startswith("parse") else Oem.from_str(source, notation)

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
    parser.add_argument("--records", type=int, nargs="+", default=[4, 10_000])
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
