# OCM 3.0 Focused Conformance Evidence

This inventory records the current message-specific evidence for standalone OCM 3.0. It is
subordinate to the [conformance policy](../conformance-policy.md) and does not promote any support
matrix cell by itself.

## Scope and authorities

- Message edition: OCM 3.0.
- Semantic and KVN authority: CCSDS 502.0-B-3 plus editorial corrigendum 1, principally sections
  6 and 7 and the OCM implementation-conformance statement in annex A.
- XML authority: NDM/XML schema set 4.0.0 and OCM schema 3.0.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and notation conversion through the shared Rust generation gate.

## Executable evidence

`just conformance-ocm` runs `ocm_conformance` and `ocm_kvn_allocations`, which establish:

| Concern | Evidence |
| --- | --- |
| KVN lexical and marked-block structure | The OCM scanner rejects non-ASCII/control input, overlong keyword records, malformed or unknown assignments, duplicate and reordered keywords, unknown/nested/mismatched blocks, invalid logical-block order, and comments outside the beginning of a block. Complete Annex G covariance history records are not split merely because they exceed the keyword-record limit. |
| XML structure | The shared XML sequence engine is registered for the complete OCM root, header, metadata, data, trajectory, physical, covariance, maneuver, perturbation, orbit-determination, and user-defined structures. It rejects unknown, duplicate, reordered, and non-schema nested content and attributes. |
| Silent-loss corrections | Complex `pert`, `od`, and `user` elements no longer use the scalar nullable adapter, so they survive XML parsing. KVN retains optional GM units. `DC_REF_DIR` and `DC_BODY_TRIGGER` use the schema's single three-number lexical value rather than generating nested Rust field elements. |
| Fixture and history preservation | All five shipped KVN fixtures and the shipped XML fixture retain their typed model under source-notation regeneration. KVN/XML crossings retain trajectory, physical, covariance, maneuver, perturbation, OD, and user blocks represented by the corpus. Every generated XML document validates against the official 4.0.0 master schema. |
| Generation boundary | Materialized and streaming KVN generation reject non-ASCII free text and overlong keyword records, while finite trajectory/covariance numbers are rounded when necessary to the CCSDS digit limit. The TIME_AND_ANGLE vector regression proves both notation paths. |
| Resource behaviour | Parsing 10 versus 1,000 trajectory and covariance records uses one owned numeric vector allocation per record without per-record reallocation. Maneuver parsing is bounded by its owned strings/vectors. Validated streaming generation has record-independent temporary allocation overhead for trajectory, covariance, and maneuver histories; materialized storage stays output-proportional. |
| Reproducible workloads | Existing `ocm_trajectory_10k`, `ocm_covariance_10k`, and `ocm_maneuver_10k` Criterion groups measure KVN/XML parse, generation, and validation. The shared family matrices add representative small-message comparisons. |
| Shared surfaces and limits | `family_contract`, Python option tests, CLI dispatch tests, and the shared generation plumbing exercise bounded parsing/generation, structured diagnostics, and Rust-core delegation. |

The two shipped KVN comments that used a Greek eta were normalized to the ASCII word `eta`; strict
KVN processing does not silently accept or regenerate the non-ASCII spelling.

## Normative inventory reconciliation

Annex A, section A2.5.4 contains the OCM implementation-conformance statement. Its logical
inventory is reconciled as follows; the counts are statement rows, not public capability cells.

| ICS group | Rows | Reconciled implementation evidence |
| --- | ---: | --- |
| Header | 7 | Version, creation date, originator, message ID, and leading comments are covered by the common ODM header model and strict root/header sequence checks. |
| Metadata | 51 | Required identity/time fields, SCLK dependencies, leap-second pairing, units, ordering, duplicates, and comments are covered by typed metadata validation plus KVN/XML structural checks. |
| Trajectory | 23 | Block metadata, composition/units cardinality, time tags, epoch ordering, fixed record shape, and representability are covered by the trajectory model and history tests. |
| Physical | 54 | The complete optional scalar/vector XML/KVN model is registered; required-field, unit, order, duplicate, and notation-preservation behavior is exercised by the fixture corpus. |
| Covariance | 18 | Block metadata, ordering, epoch sequence, composition/units, fixed record shape, and representability are covered by covariance validation and history tests. |
| Maneuver | 35 | Time-tag choice, composition/units, duty-cycle dependencies (including `TIME_AND_ANGLE` vectors), epoch sequence, and representability are covered by typed validation and regression tests. |
| Perturbation | 33 | The complete nested block is retained rather than passed through the former nullable-scalar adapter; schema order, legal attributes, units, and round trips are exercised. |
| Orbit determination | 33 | The complete nested block is retained with typed required/optional values, legal units, schema order, and fixture round trips. |
| User defined | 5 | Marked-block placement, comment placement, parameter names/values, and XML/KVN preservation are covered. |

Requirements that depend on mission truth, external identifier catalogues, or physical-model
agreement are not self-contained NDM validation and remain outside the product scope. This mapping
does not claim byte-preserving regeneration.

## Public-surface and artifact evidence

`bindings/python/tests/test_ocm.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` exercise the Python class, typed nested setters, generic
dispatch, strict parsing/generation options, and shared limits. `family_surface_cli` exercises the
actual CLI binary for OCM validation, KVN-to-XML conversion, type identity, exit status, and
zero-stdout limit failure. The repository-wide `package-python` and `package-rust` gates install
and import the built artifacts; they are release gates shared by all family cells.

## Status

OCM remains `implemented-unverified`, as required while these grouped results await review into
exact operation × notation × surface cells. The grouped inventory and shared artifact gates are
evidence for that later review; they do not themselves promote a matrix cell.
