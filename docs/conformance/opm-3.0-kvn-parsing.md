# OPM 3.0 KVN Parsing Requirement Inventory

Capability: `OPM-3.0-KVN-PARSE-RUST`

Normative authority is CCSDS 502.0-B-3 with editorial corrigendum 1, especially sections 3.2 and
7.3–7.9. KVN has no schema artifact.

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Complete OPM structure and fixed ordering | Covered | `opm_strict_kvn_parsing` rejects unknown, duplicate, reordered, trailing, and incomplete fixed content; all four Annex G fixtures parse. |
| Assignment syntax, keyword repertoire, and user-defined mechanism | Covered | Strict preflight requires one assignment per non-comment line, accepts only fixed OPM keys or the CCSDS `USER_DEFINED_` mechanism, and the existing OPM parser tests cover typed values and units. |
| Character repertoire and 254-character line bound | Covered | `opm_strict_kvn_parsing` rejects non-ASCII/control data, lone CR, and overlong lines; LF and CRLF parse to equal models. |
| Comment placement and preservation | Covered | Strict preflight permits comments only at logical-block starts; Annex G generation evidence compares logical comment positions after parsing. Leading or trailing comments that the model cannot represent are rejected rather than lost. |
| Epochs, units, numerics, optional blocks, repeated maneuvers, user data | Covered | The shipped minimal/rich fixtures plus OPM epoch, numeric, maneuver, covariance, and user-defined tests exercise the typed parser and shared validation. |
| Located bounded diagnostics | Covered | `opm_parse_diagnostics` fixes operation/notation/message/edition/code, byte/line/column, a 128-character token excerpt, expected context, field paths, and no recovery in strict mode. |
| Resource behavior | Covered | `opm_parse_limits` covers optional exact aggregate byte limits and stable resource codes; the normative line bound applies without configuration. |
| Performance and fuzzing | Partial | `kvn_parse_opm` plus early/late-invalid and configured-limit workloads are present in the existing Criterion/CodSpeed-compatible benchmark corpus. The generic KVN fuzz target reaches OPM from a checked-in minimal OPM seed, and a local smoke run completes without a crash. Published parse timing/allocation budgets and sustained CI fuzz-run evidence remain open. |

Caller-context facts such as registry membership, ICD-defined frame meaning, and physical
time-system consistency are not self-contained parse requirements and are not guessed.
