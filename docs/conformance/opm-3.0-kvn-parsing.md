# OPM 3.0 KVN Parsing Requirement Inventory

Capability: `OPM-3.0-KVN-PARSE-RUST`

Normative authority is CCSDS 502.0-B-3 with editorial corrigendum 1, especially sections 3.2 and
7.3–7.9. KVN has no schema artifact.

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Complete OPM structure and fixed ordering | Covered | `opm_strict_kvn_parsing` rejects unknown, duplicate, reordered, trailing, and incomplete fixed content; all four Annex G fixtures parse. `TRUE_ANOMALY` and `MEAN_ANOMALY` share an ordering rank so either may fill the anomaly slot, and the scanner compares keys as well as ranks so a repeated anomaly is still rejected as a duplicate while both alternatives present remain a semantic choice violation. |
| Assignment syntax, keyword repertoire, and user-defined mechanism | Covered | Strict preflight requires one assignment per non-comment line, accepts only fixed OPM keys or the CCSDS `USER_DEFINED_` mechanism, and the existing OPM parser tests cover typed values and units. |
| Character repertoire, line endings, and 254-character line bound | Covered | `opm_strict_kvn_parsing` rejects non-ASCII/control data and overlong lines; all four normative terminators (CR, LF, CRLF, and LFCR) parse to equal models. |
| Comment placement and preservation | Covered | Strict preflight permits comments only at logical-block starts; Annex G generation evidence compares logical comment positions after parsing. Leading or trailing comments that the model cannot represent are rejected rather than lost. Whitespace in a comment value is retained under the comment-specific rule in 7.8.5; the general end-of-line whitespace rule in 7.4.7 does not override it. |
| Epochs, units, numerics, optional blocks, repeated maneuvers, user data | Covered | The shipped minimal/rich fixtures plus OPM epoch, numeric, maneuver, covariance, and user-defined tests exercise the typed parser and shared validation. |
| Located bounded diagnostics | Covered | `opm_parse_diagnostics` fixes operation/notation/message/edition/code, byte/line/column, a 128-character token excerpt, expected context, and field paths. |
| Resource behavior | Covered | `opm_parse_limits` covers optional exact aggregate byte limits and stable resource codes; the normative line bound applies without configuration. |
| Performance and fuzzing | Covered | `kvn_parse_opm` plus early/late-invalid and configured-limit workloads are present in the existing Criterion/CodSpeed-compatible benchmark corpus. The generic KVN fuzz target reaches OPM from a checked-in minimal OPM seed, and the reproducible smoke command completes without a crash. Wall-clock thresholds and sustained fuzzing are optional discovery/maturity work, not deterministic pre-1.0 gates. |

Caller-context facts such as registry membership, ICD-defined frame meaning, and physical
time-system consistency are not self-contained parse requirements and are not guessed.
