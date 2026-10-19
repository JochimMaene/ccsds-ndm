# Project scope

`ccsds-ndm` handles CCSDS Navigation Data Message representation and exchange: constructing typed
messages, parsing, editing, validating, converting KVN/XML, and generating output in Rust and
Python. The Rust core owns the rules; Python exposes the same behavior.

Orbit propagation, coordinate-frame and time-scale transformations, conjunction analysis,
attitude dynamics, and evaluating interpolation algorithms are outside this scope. The library
preserves the declarations needed by applications performing those tasks.

## Feature bar

A change belongs when it solves a normal message workflow, a CCSDS correctness requirement, or a
measured bottleneck. Prefer the smallest complete design, existing shared mechanisms, and explicit
caller data over inferred operational facts. Do not add speculative options, compatibility modes,
or integrations without a concrete use case.

Every supported current message family must meet the same
[verification criteria](conformance/README.md#verification-criteria). More message names or a
passing example do not compensate for incomplete rules. Book/schema disagreements need an explicit
decision, a source, and a regression test; generation must not silently discard information.

Use reproducible representative workloads for performance changes. A faster implementation must
preserve validation, error handling, data, and the documented ownership semantics.

The [support matrix](support-matrix.md) records capabilities and limits of the implementation.
The [validation contract](design/validation-contract.md) defines enforcement and authority;
this document sets the scope for future work rather than making additional support claims.
