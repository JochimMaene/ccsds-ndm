#!/usr/bin/env python3
"""Run an optional, isolated Python comparison against ccsds-ndm 3.1.1."""

from __future__ import annotations

import argparse
import gc
import json
import statistics
import subprocess
import tempfile
import time
from pathlib import Path
from typing import Callable


ROOT = Path(__file__).resolve().parents[1]
COMPETITOR = "ccsds-ndm==3.1.1"


def large_oem(records: int) -> str:
    fixture = (ROOT / "data/kvn/oem_g11.kvn").read_text()
    first_record = (
        "2019-12-18T12:00:00.331 2789.619 -280.045 -1746.755 4.73372 -2.49586 -1.04195"
    )
    prefix, _ = fixture.split(first_record, 1)
    record = (
        "2019-12-18T12:10:00.331 "
        "2789.619 -280.045 -1746.755 4.73372 -2.49586 -1.04195\n"
    )
    return prefix + record * records


def measure(operation: Callable[[], object], iterations: int) -> float:
    operation()
    samples = []
    gc.disable()
    try:
        for _ in range(3):
            started = time.perf_counter()
            for _ in range(iterations):
                operation()
            samples.append((time.perf_counter() - started) / iterations)
    finally:
        gc.enable()
    return statistics.median(samples)


def run_workload(implementation: str, records: int) -> None:
    if implementation == "ours":
        import ccsds_ndm

        parse = ccsds_ndm.from_str

        def write_kvn(message: object) -> str:
            return message.to_kvn()

    else:
        from ccsds_ndm.mapping import NDMFileFormats
        from ccsds_ndm.ndm_io import NdmIo

        io = NdmIo()
        parse = io.from_string

        def write_kvn(message: object) -> str:
            return io.to_string(message, NDMFileFormats.KVN)

    opm = (ROOT / "data/kvn/opm_g1.kvn").read_text()
    oem = large_oem(records)
    parsed_oem = parse(oem)
    if implementation == "ours":
        retained_record = parsed_oem.segments[0].data.state_vector[records // 2]

        def edit_record() -> None:
            retained_record.x += 1.0

    else:
        retained_record = parsed_oem.body.segment[0].data.state_vector[records // 2]

        def edit_record() -> None:
            retained_record.x.value += 1.0

    results = {
        "parse OPM KVN": measure(lambda: parse(opm), 100),
        f"parse OEM {records:,} KVN": measure(lambda: parse(oem), 1),
        f"generate OEM {records:,} KVN": measure(lambda: write_kvn(parsed_oem), 1),
        f"edit retained OEM record ({records:,} total)": measure(edit_record, 100_000),
    }
    print(json.dumps(results))


def checked_run(command: list[str]) -> None:
    subprocess.run(command, cwd=ROOT, check=True)


def benchmark(python_version: str, records: int) -> None:
    wheels = sorted(
        (ROOT / "dist").glob("ccsds_ndm_py-*.whl"),
        key=lambda path: path.stat().st_mtime,
    )
    if not wheels:
        raise SystemExit("No Python wheel found. Run `just build` first.")

    with tempfile.TemporaryDirectory(prefix="ccsds-ndm-benchmark-") as directory:
        directory = Path(directory)
        ours = directory / "ours"
        competitor = directory / "competitor"
        checked_run(["uv", "venv", "--python", python_version, str(ours)])
        checked_run(["uv", "venv", "--python", python_version, str(competitor)])
        checked_run(
            [
                "uv",
                "pip",
                "install",
                "--python",
                str(ours / "bin/python"),
                str(wheels[-1]),
            ]
        )
        checked_run(
            [
                "uv",
                "pip",
                "install",
                "--python",
                str(competitor / "bin/python"),
                COMPETITOR,
            ]
        )

        def collect(python: Path, implementation: str) -> dict[str, float]:
            output = subprocess.check_output(
                [
                    str(python),
                    str(Path(__file__).resolve()),
                    "--implementation",
                    implementation,
                    "--records",
                    str(records),
                ],
                cwd=ROOT,
                text=True,
            )
            return json.loads(output)

        ours_results = collect(ours / "bin/python", "ours")
        competitor_results = collect(competitor / "bin/python", "competitor")

    print(
        f"Isolated Python {python_version}; competitor {COMPETITOR}; median of 3 runs."
    )
    print("| Workload | This library | Competitor | Lead |")
    print("|---|---:|---:|---:|")
    for workload, ours_seconds in ours_results.items():
        competitor_seconds = competitor_results[workload]
        print(
            f"| {workload} | {format_duration(ours_seconds)} | "
            f"{format_duration(competitor_seconds)} | "
            f"{competitor_seconds / ours_seconds:.0f}x |"
        )


def format_duration(seconds: float) -> str:
    if seconds < 0.000001:
        return f"{seconds * 1_000_000_000:.1f} ns"
    if seconds < 0.001:
        return f"{seconds * 1_000_000:.1f} µs"
    if seconds < 1:
        return f"{seconds * 1_000:.2f} ms"
    return f"{seconds:.2f} s"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--python", default="3.12")
    parser.add_argument("--records", type=int, default=10_000)
    parser.add_argument("--implementation", choices=["ours", "competitor"])
    arguments = parser.parse_args()

    if arguments.records < 1:
        parser.error("--records must be positive")
    if arguments.implementation:
        run_workload(arguments.implementation, arguments.records)
    else:
        benchmark(arguments.python, arguments.records)


if __name__ == "__main__":
    main()
