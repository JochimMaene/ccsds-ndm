# OPM 3.0 Conversion Requirement Inventory

Capabilities: `OPM-3.0-KVN-TO-XML-RUST` and `OPM-3.0-XML-TO-KVN-RUST`.

Conversion is the composition of strict source parsing and validated target generation in the Rust
core. It preserves OPM 3.0 and the complete supported typed meaning; it does not maintain source
formatting.

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Both directions and edition preservation | Covered | `opm_conversion::both_conversion_directions_preserve_the_complete_typed_model` covers every shipped OPM KVN fixture and the standalone XML fixture. Generation preserves the parsed edition. |
| Complete semantic equivalence | Covered | Equality of the parsed typed models covers header/metadata, epochs, state vector, Keplerian and spacecraft blocks, all covariance values and units, repeated maneuvers, comments, and user-defined parameters. Comment *position* is preserved except for the one merge KVN cannot express; see the note below. |
| Target notation validity | Covered | XML-generation evidence validates output with the official OPM 3.0 XSD. KVN-generation evidence fixes normative ordering, lexical rules, units, line bounds, and reparsing. |
| Numeric notation conversion | Covered | `xml_to_kvn_rounds_values_to_the_ccsds_digit_limit` proves that an XML value exceeding KVN's 16-significant-digit representation is rounded to a conforming KVN value. |
| String, file, and dispatch surfaces | Covered | `opm_conversion` covers typed string conversion and atomic file replacement; generation suites cover type-erased dispatch. Parse/generation diagnostic suites cover invalid source, sink, and resource failures. |
| Resource and failure behavior | Covered | Source input limits are applied by the shared core. Failed file conversion preserves the existing destination and leaves no temporary file. |
| Representative performance | Covered | The parse and generation benchmark workloads measure both halves independently without hiding model construction. Deterministic input/output and allocation gates cover resource regressions; wall-clock results are informational before a stable threshold is justified. |

No separate conversion model or rule engine exists: Python conversion calls this Rust path.

## Comment position across the two notations

The XML schema gives the data section and the state vector separate `COMMENT` positions
(`opmData` and `stateVectorType` each declare their own). The KVN keyword table has a single
comment slot ahead of `EPOCH`, so the two positions are indistinguishable once written.

The library resolves this deterministically rather than arbitrarily:

- Parsing KVN assigns every pre-`EPOCH` comment to `body.segment.data.comment` and leaves
  `body.segment.data.state_vector.comment` empty.
- Generating KVN writes `data.comment` and then `state_vector.comment`, preserving both contents
  and their relative order.

A model that came from XML with comments in both positions therefore has them merged into the
data section by a round trip through KVN. No comment is lost, reordered, or duplicated, and the
merge is idempotent. Both model fields are kept, because XML does distinguish the two positions
and a direct XML-to-XML round trip preserves them exactly.

`opm_conversion::kvn_merges_data_and_state_vector_comments_into_the_data_section` fixes this
contract.
