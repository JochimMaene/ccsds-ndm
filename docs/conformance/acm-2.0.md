# ACM 2.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone ACM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: ACM 2.0.
- Semantic and KVN authority: CCSDS 504.0-B-2, principally sections 5 and 6.
- XML authority: NDM/XML schema set 4.0.0 and ACM schema 2.0.
- Profile: strict standalone parsing, complete logical-block preservation, deterministic
  generation, and bounded notation conversion through the shared Rust generation gate.

## Executable evidence

`just conformance-acm` runs `acm_conformance` and `acm_kvn_allocations`, which establish:

| Concern | Evidence |
| --- | --- |
| KVN structure | The family-local scanner rejects non-ASCII/control input, overlong records, malformed/unknown/duplicate/reordered assignments, misplaced comments, assignments after history data, unknown/nested/mismatched blocks, invalid outer-block order/repetition, invalid SENSOR nesting, and trailing content. |
| XML structure | The shared sequence engine covers root/header/body/segment, metadata, all six data block types, nested sensor data, history records, user-defined parameters, and their legal attributes. Unknown, duplicate, reordered, and illegal-attribute content is rejected. |
| Schema-model fidelity | ACM 2.0 covariance identifiers and basis identifiers, AD Euler sequence, sensor covariance count, frequency units, CP vector units, and target-momentum vector units are modeled. Non-schema `COV_CONFIDENCE`, `AD_EPOCH`, split CP/momentum aliases, and AD `ATTITUDE_TYPE` are rejected rather than emitted or silently lost. |
| Fixture preservation | All four shipped KVN fixtures preserve their complete typed model through KVN and XML. Generated XML validates against the official 4.0.0 master schema. Schema validation runs through libxml2, which establishes structure, ordering, and lexical form; it is not evidence of numeric domain validity, because libxml2 accepts NaN against bounding facets (see the XSD oracle policy in the [validation contract](../design/validation-contract.md)). |
| Generation boundary | Materialized and streaming KVN generation reject non-ASCII/overlong text and numbers exceeding the ODM significant-digit representation before writing bytes. Fixed-shape CP, momentum, target-attitude, and attitude-history records are validated. |
| Resource behaviour | Attitude and covariance rows allocate exact numeric capacity. History generation uses allocation-free numeric-record/vector writers; streaming validation and generation use record-independent temporary storage. |
| Reproducible workloads | `acm_kvn_history_scaling` registers 100, 1,000, 10,000, and 50,000-record parse/generate workloads in the Criterion/CodSpeed-compatible KVN harness. |

## Normative inventory reconciliation

Annex A, section A2.2.3 contains 110 ACM implementation-conformance statement rows:

| ICS rows | Subject | Reconciled implementation evidence |
| --- | --- | --- |
| 1–6 | Header | Strict version-first parsing, common required header fields, leading comments, ordering, and duplicate rejection. |
| 7–29 | Metadata | Typed object/time/frame fields, data-element ordering declarations, required fields, comments, units, and strict XML/KVN order. |
| 30–43 | Attitude state history | Typed attitude/rate choices, fixed `NUMBER_STATES` record shape, epoch/value ordering, legal units, block comments, and history representability. |
| 44–58 | Physical description | Complete typed scalar/vector physical model, CP frame dependency, fixed vector shape/units, schema order, and fixture preservation. Value validation covers `DRAG_COEFF` finiteness, `WET_MASS`/`DRY_MASS` as non-negative masses, and per-component finiteness of the fixed three-element `CP`. |
| 59–68 | Covariance history | Covariance type/basis identifiers, fixed row shape/count, epochs, units, comments, and history representability. |
| 69–82 | Maneuver history | Typed maneuver metadata and records, fixed vector/state shapes, units, comments, ordering, and representability. Value validation covers a non-negative `MAN_DURATION`, the `MAN_END_TIME`/`MAN_DURATION` exclusivity, the `TARGET_MOMENTUM`/`TARGET_MOM_FRAME` pairing, and per-component finiteness of every maneuver target choice. |
| 83–105 | Attitude determination and sensors | Typed AD fields, Euler sequence, nested sensor blocks, declared sensor count/covariance dimensions, units, nesting, order, and duplicate rejection. |
| 106–110 | User defined | Block placement, leading comments, names/values, order, and XML/KVN preservation. |

The typed validators enforce required fields and locally decidable record/cardinality,
choice-dependent, frame, and fixed-shape rules. ACM's exception to the shared policy on [externally governed values](family-shared-contract.md#externally-governed-values) is
sensor-model truth, which no self-contained message rule can establish.

## Public-surface and artifact evidence

`bindings/python/tests/test_acm.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` cover every nested section's setters, typed choices,
KVN/XML parsing and generation, generic identity, and limits. `package-python` and `package-rust`
are the common built-artifact gates.

## Status

ACM remains `implemented-unverified` under the [shared promotion policy](family-shared-contract.md#promotion-policy). No family-specific promotion blocker is
known; only the exact-cell review remains.
