# OEM and OMM 3.0 Python Surface Inventory

This inventory covers the Python surface for OEM 3.0 and OMM 3.0. It delegates parsing,
validation, generation, conversion, diagnostics, and resource limits to the verified Rust core
behavior documented in `oem-3.0.md` and `omm-3.0.md`.

## Python

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Strict KVN/XML parsing | Covered | `test_verified_odm_surfaces.py` parses shipped fixtures in both notations and rejects unknown KVN keywords and XML elements with message-specific parse context. |
| Validated KVN/XML generation | Covered | The focused surface test mutates required header data and confirms that generation fails through the shared validation gate for both message types. |
| Both conversion directions | Covered | KVN→XML→KVN conversion is compared through each type's canonical KVN model representation. |
| Resource behavior | Covered | Focused tests exercise exact input and output limits; OEM history-record limits remain covered by `test_parse_and_generation_options.py`. |
| Model/API parity | Covered | `just audit` and `just stubs-check` enforce Rust-field exposure and runtime/type-stub agreement. |
| Packaged surface | Covered | `just package-python` installs the wheel in isolation and exercises OPM, OEM, and OMM parsing and conversion in both notations. |

The adapter contains no independent CCSDS rules. Its conformance claim is therefore the verified
core behavior plus evidence that the public adapter preserves that behavior and its failures.
