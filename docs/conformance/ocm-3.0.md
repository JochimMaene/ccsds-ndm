# OCM 3.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone OCM 3.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

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
| Fixture and history preservation | All five shipped KVN fixtures and the shipped XML fixture retain their typed model under source-notation regeneration. KVN/XML crossings retain trajectory, physical, covariance, maneuver, perturbation, OD, and user blocks represented by the corpus. Every generated XML document validates against the official 4.0.0 master schema. Schema validation runs through libxml2, which establishes structure, ordering, and lexical form; it is not evidence of numeric domain validity, because libxml2 accepts NaN against bounding facets (see the XSD oracle policy in the [validation contract](../design/validation-contract.md)). |
| Generation boundary | Materialized and streaming KVN generation reject non-ASCII free text and overlong keyword records, while finite trajectory/covariance numbers are rounded when necessary to the CCSDS digit limit. The TIME_AND_ANGLE vector regression proves both notation paths. |
| Resource behaviour | Parsing 10 versus 1,000 trajectory and covariance records uses one owned numeric vector allocation per record without per-record reallocation. Maneuver parsing is bounded by its owned strings/vectors. Validated streaming generation has record-independent temporary allocation overhead for trajectory, covariance, and maneuver histories; materialized storage stays output-proportional. |
| Reproducible workloads | Existing `ocm_trajectory_10k`, `ocm_covariance_10k`, and `ocm_maneuver_10k` Criterion groups measure KVN/XML parse, generation, and validation. The shared family matrices add representative small-message comparisons. |
| Shared surfaces and limits | `family_contract`, Python option tests, and the shared generation plumbing exercise bounded parsing/generation, structured diagnostics, and Rust-core delegation. |

The two shipped KVN comments that used a Greek eta were normalized to the ASCII word `eta`; strict
KVN processing does not silently accept or regenerate the non-ASCII spelling.

## Normative inventory reconciliation

Annex A, section A2.5.4 contains the OCM implementation-conformance statement. Its logical
inventory is reconciled as follows; the counts are statement rows, not public capability cells.

| ICS group | Rows | Reconciled implementation evidence |
| --- | ---: | --- |
| Header | 7 | Version, creation date, originator, message ID, and leading comments are covered by the common ODM header model and strict root/header sequence checks. |
| Metadata | 51 | Required identity/time fields, SCLK dependencies, leap-second pairing, units, ordering, duplicates, and comments are covered by typed metadata validation plus KVN/XML structural checks. |
| Trajectory | 23 | Block metadata, composition/units cardinality, time tags, epoch ordering, fixed record shape, and representability are covered by the trajectory model and history tests. `validate_ocm_line_values` additionally rejects non-finite numbers in every `trajLine`, naming the failing line and column. |
| Physical | 54 | The complete optional scalar/vector XML/KVN model is registered; required-field, unit, order, duplicate, and notation-preservation behavior is exercised by the fixture corpus. `OcmPhysicalDescription::validate` covers the block's own editable values: `DRAG_COEFF_NOM`, the eleven areas, three masses, two percentages, `REFLECTANCE`, the three attitude angles, and the remaining plain doubles for finiteness. |
| Covariance | 18 | Block metadata, ordering, epoch sequence, composition/units, fixed record shape, and representability are covered by covariance validation and history tests, and `validate_ocm_line_values` rejects non-finite numbers in every `covLine`. |
| Maneuver | 35 | Time-tag choice, composition/units, duty-cycle dependencies (including `TIME_AND_ANGLE` vectors), epoch sequence, and representability are covered by typed validation and regression tests. |
| Perturbation | 33 | The complete nested block is retained rather than passed through the former nullable-scalar adapter; schema order, legal attributes, units, and round trips are exercised. `OcmPerturbations::validate` covers `GM`, `OBLATE_FLATTENING`, `ALBEDO_GRID_SIZE`, and the finiteness of `EQUATORIAL_RADIUS`, `CENTRAL_BODY_ROTATION`, and the geomagnetic and solar-flux values. |
| Orbit determination | 33 | The complete nested block is retained with typed required/optional values, legal units, schema order, and fixture round trips. `OcmOdParameters::validate` covers the positive-integer counts, `GDOP` and `WEIGHTED_RMS` (including NaN and both infinities), the OD spans, the eigenvalue lengths, `OD_CONFIDENCE`, and `SEDR`. |
| User defined | 5 | Marked-block placement, comment placement, parameter names/values, and XML/KVN preservation are covered. |

These OCM values are deliberately **not** validated, each for a recorded reason rather than by
omission:

- `DAYS_SINCE_FIRST_OBS`, `DAYS_SINCE_LAST_OBS`, `DC_PA_START_ANGLE`, and `DC_PA_STOP_ANGLE` are
  validated for finiteness only, because ODM's domain is wider than the 3.0 schema's. The
  book-valid value is preserved and KVN writes it; XML generation refuses the conversion with the
  offending field path rather than altering the value. Both accepted boundaries generate
  schema-valid XML.
- `OEB_Q1`–`OEB_QC` are not checked for unit norm, because ODM assigns `-999` as the
  tumbling-object flag, so no norm rule exists.
- `OEB_MAX`/`OEB_INT`/`OEB_MIN` ordering — the book's wording is descriptive, not a `shall`.
- Metadata `TIME_SPAN` is deliberately not compared against `START_TIME`/`STOP_TIME`. ODM table
  6-3 defines it as their difference, but the operands may carry different time systems, and the
  block does not supply the context needed to compare them. Enforcing it would risk rejecting
  valid messages, so the rule is recorded rather than implemented.
- `MAN_*` line values remain `Vec<String>` because the columns are heterogeneous, but every
  column that `MAN_COMPOSITION` declares numeric must now hold a finite number. `ACC_INTERP`,
  `THR_INTERP`, and `DEPLOY_ID` are the only non-numeric columns in ODM tables 6-8 and 6-9. No
  per-column domain is imposed, matching Orekit's `ManeuverFieldType`. The XSD types the line as a
  string list, so the schema oracle cannot establish this rule and a direct test does.

OCM's exception to the shared policy on [externally governed values](family-shared-contract.md#externally-governed-values) is that its `OBJECT_TYPE` is delegated to the living
SANA Object Types registry, so the token is preserved losslessly and checked for offline syntax
rather than current membership — unlike the closed CDM and RDM lists. This mapping does not claim
byte-preserving regeneration.

## Public-surface and artifact evidence

`bindings/python/tests/test_ocm.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` exercise the Python class, typed nested setters, generic
dispatch, strict parsing/generation options, and shared limits. The repository-wide
`package-python` and `package-rust` gates verify the built artifacts; they are release gates shared
by all family cells.

## Status

OCM remains `implemented-unverified` under the [shared promotion policy](family-shared-contract.md#promotion-policy). Its family-specific open items are the five
deliberately unvalidated values listed above, of which `DAYS_SINCE_*_OBS` and the phase-angle range
are book/XSD conflicts rather than gaps.
