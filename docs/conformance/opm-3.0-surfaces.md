# OPM 3.0 Python Surface Inventory

This inventory covers the OPM 3.0 Python surface. It is a thin adapter over the verified Rust
parsing, validation, generation, conversion, diagnostic, and resource behavior.

## Python

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Strict KVN/XML parse and validation | Covered | Python tests parse both notations, reject semantic and syntactic violations, expose aggregate validation, and verify structured exception attributes and input limits. |
| KVN/XML generation | Covered | Python tests exercise source-edition defaults, explicit edition selection, notation choice, output limits, and atomic file behavior through the Rust generation gate. |
| Both conversion directions | Covered | `test_python_conversion_delegates_to_strict_rust_core` performs KVN→XML→KVN through the module conversion API and compares canonical typed results; file conversion tests success and destination preservation on failure. |
| Model/API parity | Covered | `just audit` and `just stubs-check` enforce Rust-field exposure and runtime/type-stub agreement. |
| Packaged surface | Covered | `just package-python` builds the ABI3 wheel, installs it with dependencies in an isolated environment, and imports OPM plus conversion APIs. |

The adapter contains no independent CCSDS conformance logic. Platform availability beyond the
explicitly tested release environments is not implied by these cells.
