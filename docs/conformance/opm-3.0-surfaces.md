# OPM 3.0 Python and CLI Surface Inventory

This inventory covers the exact OPM 3.0 Python and CLI cells in the support matrix. Both surfaces
are thin adapters over the verified Rust parsing, validation, generation, conversion, diagnostic,
and resource behavior.

## Python

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Strict KVN/XML parse and validation | Covered | Python tests parse both notations, reject semantic and syntactic violations, expose aggregate validation, and verify structured exception attributes and input limits. |
| KVN/XML generation | Covered | Python tests exercise source-edition defaults, explicit edition selection, notation choice, output limits, and atomic file behavior through the Rust generation gate. |
| Both conversion directions | Covered | `test_python_opm_conversion_delegates_to_strict_rust_core` performs KVN→XML→KVN through the module conversion API and compares canonical typed results; file conversion tests success and destination preservation on failure. |
| Model/API parity | Covered | `just audit-strict`, `just stubs-check`, and `just sync-docs-check` enforce Rust-field exposure, runtime/type-stub agreement, and Rust-derived documentation. |
| Packaged surface | Covered | `just package-python` builds the ABI3 wheel, installs it with dependencies in an isolated environment, and imports OPM plus conversion APIs. The exact tested environment and broader compatibility scope are stated in the [Python release policy](../python-release-policy.md). |

## CLI

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| KVN and XML validation | Covered | `opm_cli::validate_has_stable_exit_and_json_diagnostic_contracts` validates both notations and fixes invalid-input and resource-limit exits plus machine-readable diagnostic fields. |
| Both conversion directions | Covered | `opm_cli::convert_keeps_document_bytes_separate_and_protects_destination_files` performs KVN→XML→KVN and compares complete parsed models. |
| Pipeline contract | Covered | CLI tests cover stdin/stdout separation, diagnostics on stderr, stable exits, explicit formats and target edition, and preservation of an existing destination after failure. |
| Packaged surface | Covered | `just package-rust` installs the CLI from the extracted crate artifact and executes it in the tested current-stable Ubuntu environment. |

Neither adapter contains independent CCSDS conformance logic. Platform availability beyond the
explicitly tested release environments is not implied by these cells.
