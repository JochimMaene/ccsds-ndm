# AEM 2.0 conformance

This inventory records maintainer evidence for standalone AEM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: AEM 2.0.
- Semantic and KVN authority: CCSDS 504.0-B-2, principally sections 4 and 6.
- XML authority: NDM/XML schema set 4.0.0 and AEM schema 2.0.
- Profile: strict standalone parsing, typed attitude-state validation, deterministic generation,
  and bounded notation conversion through the shared Rust generation gate.

## Executable evidence

The `aem`, `aem_kvn_allocations`, and `library::output` suites establish:

| Concern | Evidence |
| --- | --- |
| KVN structure | The family-local marked-block scanner rejects non-ASCII/control input, overlong records, malformed/unknown/duplicate/reordered metadata, misplaced history comments, unknown/nested/mismatched blocks, invalid META/DATA sequencing, assignments in history data, and trailing content. It emits and accepts the table 4-3 keyword `ANGVEL_FRAME` in both notations. The previously emitted KVN `RATE_FRAME` remains a read alias; supplying both spellings is a duplicate, not two independent fields. |
| XML structure and choices | The shared XML sequence engine covers the complete root/header/body/segment hierarchy and every nested quaternion, derivative, angular-velocity, Euler, spin, nutation, and momentum structure. It rejects unknown, duplicate, reordered, multi-choice, and non-schema attribute content. |
| Fixture and typed-state preservation | Both KVN fixtures preserve their complete typed model through KVN and XML. Both XML fixtures preserve their XML model; the all-types fixture covers all nine attitude-state choices. Generated XML validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| Optional fixed units | AEM KVN history records forbid unit annotations (504.0-B-2 section 6.9.2) while the fixed XML unit attributes are optional (section 7.6.10). Optional fixed XML units on derivative, angle, rate, nutation, and momentum values are therefore deliberately normalized to omission on an XML-to-KVN-to-XML hop, while values, choices, and record counts remain preserved. A dedicated regression verifies the output stays schema-valid. |
| History numbers | Reading takes the CCSDS number shape but not its 16-digit cap or 32-bit integer range, so the 17-digit shortest spelling of a double parses; this is the same lenient reading as [OEM](oem-3.0.md#more-lenient-than-the-book). Generation still rounds to 16 significant digits. |
| Generation boundary | Materialized and streaming KVN generation reject non-ASCII/overlong text and history numbers exceeding the ODM significant-digit representation before writing bytes. |
| Resource behaviour | History parsing uses a fixed eight-number stack buffer rather than a heap vector per record; owned epoch/state storage remains linear. Validation and streaming generation avoid per-record temporary allocation. Materialized output remains output-proportional. |
| Reproducible workloads | `aem_kvn_history_scaling` registers 100, 10,000 and 50,000-record parse/generate workloads and `aem_xml_history_scaling` the first two, the sizes every history-carrying family shares. |
| Self-contained timeline semantics | Metadata requires an ordered total span; usable bounds must lie within it and be ordered. Every state epoch must lie within the total span and state epochs must be strictly increasing without repetition. Adjacent blocks cannot move usable time backwards. Invalid timelines fail materialized and streaming generation before output. |

## Normative inventory reconciliation

Annex A, section A2.2.2 contains 25 AEM implementation-conformance statement rows:

| ICS rows | Subject | Reconciled implementation evidence |
| --- | --- | --- |
| 1–6 | Header | Strict version-first root/header parsing, required common header fields, leading comments, ordering, and duplicates. |
| 7–24 | Metadata and block structure | Required object/frame/time/type fields; optional Euler/rate metadata and conditional interpolation degree; `META`/`DATA` sequencing; total/usable spans; strict epoch ordering; comments; units; and repeatable segments. |
| 25 | Attitude ephemeris data | All nine table 4-4 attitude-state alternatives are typed and exclusive, have fixed record widths/order/units, preserve their epochs and values, and are exercised by the all-types XML fixture. Root validation additionally revisits every branch and every record, not only index zero: finite quaternion components in `[-1, 1]` on the three quaternion branches, angle bounds on all Euler and spin angles, a non-negative, seconds-only `NUTATION_PER`, and finiteness of every rate, derivative, and momentum component. This is distinct from the KVN significant-digit representability preflight, which is a different rule at a different boundary. |

Section 4.2.4.8.1's strictly increasing, non-repeated epoch requirement and the metadata/cross-block
usable-span rules have direct semantic regression tests in `aem::validation`.

## Decisions and limits

**Optional interpretation metadata.** ADM table 4-3 and the bundled AEM schema mark
`EULER_ROT_SEQ` and `ANGVEL_FRAME` optional, even for Euler/angular-velocity records.
Their applicability descriptions do not make them conditionally mandatory. Readers and edited
models therefore accept omission and preserve it through both notations; no sequence or frame is
invented. A consumer that needs those values to interpret or transform the attitude must obtain
them from its exchange context. `aem::parsing::aem_uses_canonical_angular_velocity_frame_and_preserves_optional_metadata`
covers both omissions and the old frame-keyword alias.

**Nutation periods use seconds.** ADM §4.2.4.6 and the XSD `durationType`/`timeUnits`
require seconds. The shared Rust `TimeUnits` enum also contains `Day` for other contexts,
but an AEM `NUTATION_PER` annotated with days is rejected at input, validation and output,
including before values are flattened into unitless KVN records. It is never converted implicitly.
`aem::generation::aem_nutation_periods_reject_day_units_before_values_are_flattened` fixes this boundary.

**Interpolation feasibility is a consumer responsibility.** ADM §4.2.4.8.4 requires
sufficient records for the recommended interpolation method. It neither standardizes the
algorithm behind an open `INTERPOLATION_METHOD` string nor defines a record-count formula.
The published G4 example recommends Hermite degree 7 with four quaternion-only records;
using `degree + 1` as a universal rejection rule would reject that example and impose an
algorithm the book does not specify. Hermite methods may use derivative constraints, and
an exchange-partner method may supply additional context.

The parser therefore preserves the recommendation, requires `INTERPOLATION_DEGREE` when
a method is present, and checks its positive integer domain. Root validation checks the
record shapes, choices, numeric domains, total/usable spans, and strict timeline order.
It does **not** certify that a consumer's chosen interpolator can reconstruct the attitude
throughout the usable interval. Consumers must establish that separately before interpolation.
`aem::validation::interpolation_recommendations_are_preserved_without_guessing_an_algorithm`
fixes this boundary in both notations. This is a resolved validation boundary, consistent
with OEM's interpolation policy, rather than an unresolved numeric capacity rule.

**Legacy edition is not inferred.** ADM annex J documents incompatible AEM 1.0 changes,
including quaternion order/direction keywords, rotation-sequence spelling, and attitude
rate types. Relabelling a 2.0 fixture does not verify 1.0. Input edition 1.0 is rejected
until its actual grammar and semantics receive an edition-specific implementation and review.

## Quaternion magnitude

ADM annex F defines a rotation quaternion through sine/cosine components of a rotation axis and
angle, implying unit magnitude. APM, AEM, and known quaternion ACM histories use the same shared
validator: finite components in `[-1, 1]` and squared norm in `[0.999, 1.001]`. The book gives no
numeric tolerance; this interval is an implementation choice to allow rounded published samples.
The library rejects values outside it and never silently normalizes caller data. The common
policy and its source are recorded in the [validation contract](../design/validation-contract.md).

## Public-surface and artifact evidence

`bindings/python/tests/test_aem.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` cover typed choices and NumPy construction, KVN/XML
parsing/generation, generic identity, shared limits, and core-error propagation. `package-python`
and `package-rust` provide the common built-artifact gates.

## Verification outcome

AEM 2.0 meets the common [verification criteria](README.md#verification-criteria) for
self-contained format and model validation on Rust and Python. The interpolation decision
above is part of that scope; no interpolation algorithm is implemented or certified.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).
