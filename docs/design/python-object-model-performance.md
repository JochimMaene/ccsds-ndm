# Python object-model performance baseline

Recorded with `just bench-python-object-model` on Python 3.12.7, Linux x86-64 under WSL2, on an
Intel Core i7-1165G7. These are machine-local observations, not release thresholds, and they do
not support any general claim about the library's performance.

## Reading these numbers

Three things must match before two runs are comparable, and getting any of them wrong changes the
result by more than the effect most changes are trying to measure:

- **Build profile.** These figures are from `maturin develop --release`. The same run against a
  debug build reports `parse` at 1,168 ms rather than 74 ms — roughly 15× — so a debug measurement
  is not a slower version of this table, it is a different quantity.
- **Machine load.** This box drifts by roughly ±35% depending on background load; the run below was
  taken at a load average near 4, not on an idle machine. Compare against a control measured in the
  same session rather than against this table.
- **Warm-up.** The harness performs three untimed calls before sampling, so one-off import
  resolution, allocator arena growth, and CPU frequency ramp do not land in the samples. A harness
  without warm-up reports a noticeably slower first sample and a wider spread.

| OEM records | document | operation | median | process peak RSS |
| ---: | ---: | --- | ---: | ---: |
| 4 | 2,359 B (XML) | parse XML | 0.043 ms | 25,856 KiB |
| 4 | 2,359 B (XML) | generate XML | 0.017 ms | 25,856 KiB |
| 4 | 1,067 B (KVN) | parse KVN | 0.008 ms | 25,856 KiB |
| 4 | 1,067 B (KVN) | generate KVN | 0.006 ms | 25,856 KiB |
| 4 | 2,359 B (XML) | validate/reconstruct | 0.001 ms | 25,856 KiB |
| 4 | 2,359 B (XML) | retained edit | 0.0002 ms | 25,856 KiB |
| 4 | 2,359 B (XML) | edit and validate | 0.002 ms | 25,856 KiB |
| 10,000 | 3,480,967 B (XML) | parse XML | 74.049 ms | 38,392 KiB |
| 10,000 | 3,480,967 B (XML) | generate XML | 30.253 ms | 47,568 KiB |
| 10,000 | 810,743 B (KVN) | parse KVN | 5.488 ms | 38,392 KiB |
| 10,000 | 810,743 B (KVN) | generate KVN | 3.728 ms | 38,392 KiB |
| 10,000 | 3,480,967 B (XML) | validate/reconstruct | 0.973 ms | 38,392 KiB |
| 10,000 | 3,480,967 B (XML) | retained edit | 0.0002 ms | 38,392 KiB |
| 10,000 | 3,480,967 B (XML) | edit and validate | 0.664 ms | 38,392 KiB |

## What the harness actually measures

Each operation runs in a separate process; RSS is that process's absolute peak, not incremental
memory.

The large document is *built* by repeating one child wrapper, which keeps construction cheap. **That
does not carry into any measurement.** Each worker reads the serialized document from disk and
parses it, so every timed operation — parse, generate, validate, and both edit paths — works on its
own object graph of 10,000 independent records. An earlier version of this note claimed the shared
wrapper limited what was measured; that was wrong, and it understated what the parse and generate
figures cover.

The one thing the table genuinely does not isolate is the cost of a caller *constructing* 10,000
distinct wrappers in Python by hand, since no measured operation does that.

`retained_edit` is a live-object attribute write and reflects the mutable-object design being
preserved; `edit_and_validate` adds a full root validation, which is where reconstruction cost
appears.

## What is not covered

- No allocation counts. The Rust suite carries allocation budget tests for the history-bearing
  families; this harness measures wall clock and peak RSS only.
- One message family. OEM was chosen because it is history-bearing and has both notations; the
  figures say nothing about OCM, TDM, or CDM.
- No comparison against another implementation.
