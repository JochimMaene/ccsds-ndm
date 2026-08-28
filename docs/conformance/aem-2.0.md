# AEM 2.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone AEM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: AEM 2.0.
- Semantic and KVN authority: CCSDS 504.0-B-2, principally sections 4 and 6.
- XML authority: NDM/XML schema set 4.0.0 and AEM schema 2.0.
- Profile: strict standalone parsing, typed attitude-state validation, deterministic generation,
  and bounded notation conversion through the shared Rust generation gate.

## Executable evidence

`just conformance-aem` runs `aem_conformance` and `aem_kvn_allocations`, which establish:

| Concern | Evidence |
| --- | --- |
| KVN structure | The family-local marked-block scanner rejects non-ASCII/control input, overlong records, malformed/unknown/duplicate/reordered metadata, misplaced history comments, unknown/nested/mismatched blocks, invalid META/DATA sequencing, assignments in history data, and trailing content. It emits and accepts the normative KVN `RATE_FRAME`; `ANGVEL_FRAME` remains the XML element only. |
| XML structure and choices | The shared XML sequence engine covers the complete root/header/body/segment hierarchy and every nested quaternion, derivative, angular-velocity, Euler, spin, nutation, and momentum structure. It rejects unknown, duplicate, reordered, multi-choice, and non-schema attribute content. |
| Fixture and typed-state preservation | Both KVN fixtures preserve their complete typed model through KVN and XML. Both XML fixtures preserve their XML model; the all-types fixture covers all nine attitude-state choices. Generated XML validates against the official 4.0.0 master schema. |
| Optional fixed units | AEM KVN history records forbid unit annotations. Optional fixed XML units on derivative, angle, rate, nutation, and momentum values are deliberately normalized to omission on an XML-to-KVN-to-XML hop, while values, choices, and record counts remain preserved. |
| Generation boundary | Materialized and streaming KVN generation reject non-ASCII/overlong text and history numbers exceeding the ODM significant-digit representation before writing bytes. |
| Resource behaviour | History parsing uses a fixed eight-number stack buffer rather than a heap vector per record; owned epoch/state storage remains linear. Validation and streaming generation avoid per-record temporary allocation. Materialized output remains output-proportional. |
| Reproducible workloads | `aem_kvn_history_scaling` registers 100, 1,000, 10,000, and 50,000-record parse/generate workloads in the Criterion/CodSpeed-compatible KVN harness. |
| Self-contained timeline semantics | Metadata requires an ordered total span; usable bounds must lie within it and be ordered. Every state epoch must lie within the total span and state epochs must be strictly increasing without repetition. Adjacent blocks cannot move usable time backwards. Invalid timelines fail materialized and streaming generation before output. |

## Normative inventory reconciliation

Annex A, section A2.2.2 contains 25 AEM implementation-conformance statement rows:

| ICS rows | Subject | Reconciled implementation evidence |
| --- | --- | --- |
| 1–6 | Header | Strict version-first root/header parsing, required common header fields, leading comments, ordering, and duplicates. |
| 7–24 | Metadata and block structure | Required object/frame/time/type fields; conditional Euler/rate/interpolation metadata; `META`/`DATA` sequencing; total/usable spans; strict epoch ordering; comments; units; and repeatable segments. |
| 25 | Attitude ephemeris data | All nine table 4-4 attitude-state alternatives are typed and exclusive, have fixed record widths/order/units, preserve their epochs and values, and are exercised by the all-types XML fixture. |

Section 4.2.4.8.1's strictly increasing, non-repeated epoch requirement and the metadata/cross-block
usable-span rules have direct semantic regression tests in `aem_semantic_validation`.

There is one normative-example conflict that the library resolves in favor of accepting the
published CCSDS fixture: `ccsds-ndm/data/kvn/aem_g4.kvn` declares interpolation degree 7 but contains four
records. Enforcing “at least degree + 1 records” would reject that official example. The degree
field and its conditional presence are validated, but record-count capacity is therefore not used
as a rejection rule. This exception is explicit and must be resolved before any affected semantic
cell is promoted.

## Public-surface and artifact evidence

`bindings/python/tests/test_aem.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` cover typed choices and NumPy construction, KVN/XML
parsing/generation, generic identity, shared limits, and core-error propagation. `package-python`
and `package-rust` provide the common built-artifact gates.

## Status

AEM remains `implemented-unverified`. In addition to reviewed exact-cell decomposition, the
interpolation-degree/example conflict above blocks promotion of the affected semantic claim.
