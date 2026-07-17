# OPM 3.0 Conversion Requirement Inventory

Capabilities: `OPM-3.0-KVN-TO-XML-RUST` and `OPM-3.0-XML-TO-KVN-RUST`.

Conversion is the composition of strict source parsing and validated target generation in the Rust
core. It preserves OPM 3.0 and the complete supported typed meaning; it does not maintain source
formatting.

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Both directions and edition preservation | Covered | `opm_conversion::both_conversion_directions_preserve_the_complete_typed_model` covers every shipped OPM KVN fixture and the standalone XML fixture. Default generation preserves OPM 3.0; unsupported explicit target editions fail. |
| Complete semantic equivalence | Covered | Equality of the parsed typed models covers header/metadata, epochs, state vector, Keplerian and spacecraft blocks, all covariance values and units, repeated maneuvers, comments by logical position, and user-defined parameters. |
| Target notation validity | Covered | XML-generation evidence validates output with the official OPM 3.0 XSD. KVN-generation evidence fixes normative ordering, lexical rules, units, line bounds, and reparsing. |
| Information-loss rejection | Covered | `xml_to_kvn_rejects_values_that_cannot_be_represented_without_rounding` proves that an XML value exceeding KVN's 16-significant-digit representation fails with a stable field diagnostic instead of rounding. |
| String, file, and dispatch surfaces | Covered | `opm_conversion` covers typed string conversion and atomic file replacement; generation suites cover type-erased dispatch. Parse/generation diagnostic suites cover invalid source, unsupported target, sink, and resource failures. |
| Resource and failure behavior | Covered | Source input and target output limits are applied by the shared core. Failed file conversion preserves the existing destination and leaves no temporary file. |
| Representative performance | Covered | The parse and generation benchmark workloads measure both halves independently without hiding model construction. Deterministic input/output and allocation gates cover resource regressions; wall-clock results are informational before a stable threshold is justified. |

No separate conversion model or rule engine exists: Python and CLI conversion call this Rust path.
