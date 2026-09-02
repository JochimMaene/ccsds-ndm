# OPM 3.0 Typed-Model Validation Inventory

This inventory covers notation-neutral, self-contained validation of a public `Opm` model for
CCSDS 502.0-B-3 plus editorial corrigendum 1. XML character legality and KVN lexical/line rules are
checked only when that notation is parsed or generated. Registry membership and exchange-agreement
semantics require caller context and are not guessed.

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Root identity and supported edition | Covered | `opm_validation` and the generation conformance suites mutate the public root fields and fix stable paths. |
| Header and metadata required values and epochs | Covered | `opm_validation`, `opm_3_xml_generation_conformance`, and the epoch suites cover empty required text and every OPM epoch position. Originator, center, frame, and time-system registry membership is caller context. |
| State-vector values and units | Covered | XML generation mutation tests exercise every component; shared wrapper and KVN conformance tests cover allowed units and finite values. |
| Keplerian completeness, anomaly choice, ranges, and units | Covered | `opm_keplerian_xml_generation` and OPM model unit tests cover exactly one anomaly, finite/ranged values, and units. |
| Spacecraft values and units | Covered | `opm_3_xml_generation_conformance` mutates every public spacecraft value and exercises zero boundaries. |
| Covariance completeness, finite values, frame, and units | Covered | XML/KVN conformance tests cover the complete lower triangle, optional frame, units, and every non-finite component. |
| Maneuver completeness, mass dependency, ranges, units, and repetition | Covered | `opm_maneuver_duration_units`, XML mutation tests, strict parser tests, and conversion round trips cover the block and repeated maneuvers. |
| User-defined names and content | Covered | Shared model tests validate required content; notation-boundary tests apply KVN keyword rules without imposing them on XML/model validation. ICD-defined parameter meaning remains caller context. |
| Diagnostic ordering | Covered | `Validate::validate` fails at the first model-order error. `opm_validation` fixes this contract. Work is linear in the supplied model, including repeated collections. |
| Offline/caller-context behavior | Covered | Core validation performs no registry or network lookup. Context-dependent membership and mission plausibility are documented as outside self-contained validation; no unproven context API is added. |
| Notation separation | Covered | `opm_validation` is notation-neutral. XML-safe text is enforced by the OPM XML generation hook and XML parser; KVN lexical checks remain in `ToKvn::validate_kvn` and the strict KVN parser. |
| Representative cost | Covered | `opm_validate/{valid_rich,invalid_aggregate,valid_1000_maneuvers}` uses the existing Criterion/CodSpeed-compatible benchmark harness. The workload is reproducible; wall-clock results are informational under the proportionate pre-1.0 policy. |

The inventory is implementation evidence, not a conformance claim. The support matrix controls the
advertised status.
