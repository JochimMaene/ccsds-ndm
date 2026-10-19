# CDM 1.0 conformance

This inventory records maintainer evidence for standalone CDM 1.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: CDM 1.0.
- Semantic and KVN authority: CCSDS 508.0-B-1 with editorial corrigendum 2 and
  technical corrigendum 2 (`data/cdm_508x0b1e2c2.pdf`), principally sections 3 and 6 and annex D.
- XML authority: NDM/XML schema set 4.0.0 and CDM schema 1.0.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and loss-rejecting notation conversion through the shared Rust generation gate.

## Executable evidence

The `cdm` suite establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and logical structure | The family-local scanner rejects non-ASCII/control input, overlong records, malformed assignments, unknown and duplicate keywords, fixed-order violations, nonstandard marked blocks, misplaced comments, extra segments, and trailing content. It models the header, relative data, two metadata sections, and each flattened OD, additional-parameter, state-vector, and covariance block in CDM order. |
| XML structure | The shared XML sequence engine is registered for the complete CDM root, header, body, relative-state vector, two segments, metadata, data, OD, additional-parameter, state-vector, and covariance hierarchy. It rejects unknown, duplicate, reordered, and non-schema children and attributes, including the non-XSD `nil` spelling previously accepted by a permissive test. |
| Delimiter-free COMMENT decision | A leading KVN comment run before the first data keyword is normatively retained on the outer data block; no textual delimiter exists from which to infer a split. If the typed model populates the first present nested block's comments, every materialized and streaming KVN generation route rejects the state before output. The shipped XML fixture demonstrates this richer, XML-only association and is therefore intentionally not convertible to KVN. |
| Typed preservation and generation | All three shipped KVN fixtures retain their complete typed model through deterministic KVN and XML generation. The shipped XML fixture retains its model through XML generation. Every generated XML document validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| KVN fixed-point preflight | CDM's shape is a fixed, bounded set of scalar records. A private generation preflight writes and reparses that bounded representation before caller-visible output, rejecting lexical violations, multiline normalization, comment reassociation, or any other typed-model change. Streaming rejection is proven to write zero bytes. |
| KVN numeric precision | Sections 6.3.2.2 and 6.3.2.3(b) limit ordinary KVN numbers to 16 decimal digits. All numeric fields use the shared number writer; CDM additionally retains its exact-model preflight, so a value such as `0.30000000000000004` is refused in KVN if the required spelling changes its stored `f64`. XML retains it as an XML Schema double. `cdm_kvn_refuses_numeric_rounding_while_xml_retains_the_value` covers a 16-digit control, scalar/unit-bearing fields, schema-valid XML preservation, and zero-byte KVN refusals. |
| Shared surfaces | `library::parsing` and `library::output`, Python API tests, and the shared generation plumbing exercise parsing, generation, structured diagnostics, and Rust-core delegation. No separate CDM semantics are implemented in an adapter. |

## Normative inventory reconciliation

Annex A, section A2.1.5 contains 68 CDM implementation-conformance statement rows. They map to the
wire/model evidence in these groups:

| ICS rows | Subject | Reconciled implementation evidence |
| --- | --- | --- |
| 1–7 | Header | Strict version-first parsing, common header required fields, comment placement, ordering, and duplicate rejection. |
| 8–19 | Relative metadata/data | Typed relative fields and relative state vector, required-field/unit handling, exact XML order, and fixed KVN order. `RelativeMetadataData::validate` requires finite, non-negative `MISS_DISTANCE` and `RELATIVE_SPEED` because table 3-2 defines them as norms; all six relative state-vector components and `SCREEN_VOLUME_X/Y/Z` must be finite, and `COLLISION_PROBABILITY` is bounded to `[0, 1]`. `cdm_distance_and_speed_norms_require_nonnegative_scalars` covers negative input in both notations, edited models, zero boundaries, and zero-byte output refusal; Python checks the same scalar constraints. |
| 20–39 | Object metadata, repeated for OBJECT1 and OBJECT2 | Exactly two segments and one of each object designator, typed identity/orbit/covariance-method fields, required values, units, and ordering. |
| 40–58 | Object data, OD, and additional parameters | Complete flattened KVN ordering and nested XML structure, required and optional typed fields, units, and the documented delimiter-free comment decision. The shared `OdParameters::validate` and the additional-parameter validators are routed from `CdmData::validate` for both segments, covering counts, spans, percentages, masses, areas, and coefficients; signed thrust acceleration remains permitted while required to be finite. |
| 59–62 | State vector | All six required components, fixed units, ordering, and generated-schema validation. |
| 63–68 | Covariance matrix | Required lower-triangular RTN terms, optional drag/SRP/thrust extensions, fixed units, ordering, and generated-schema validation. |

The parser enforces the complete structural inventory and the self-contained
two-object/cardinality and required-field rules represented by the typed model. CDM's exceptions to
the shared policy on [externally governed values](../design/validation-contract.md#registry-governed-values) are externally established object identity and the choice of collision
probability methodology; note that `OBJECT_TYPE` is *not* an exception, being a closed list whose
unknown values are rejected.

## Physical consistency boundary

Normative table 3-2 defines miss distance and relative speed as the norms of the corresponding
relative vectors; informative annex E repeats those meanings. Their scalar values must be finite
and non-negative even though the XSD's `lengthTypeUO` and `dvType` extend unrestricted
`xsd:double`. The library does not additionally compare them to the supplied vector norms.
The published §3.6.3 KVN example retained as `cdm_363.kvn` gives `MISS_DISTANCE = 715 [m]` with
relative position `(27.4, -70.2, 711.8) [m]`, whose norm is approximately `715.777926 m`.
Strict equality would reject this book example; the book specifies no comparison tolerance.
The library preserves the supplied values rather than inventing a tolerance or recomputing one
field from another. This is an explicit limit of validation of the book-defined relationship.

The library also does not compare screening entry/exit or start/stop times or test covariance
matrices for positive-semidefiniteness. Screening sizes and covariance entries use finite-double
domains without additional physical sign checks. It bounds collision probability to `[0, 1]`
but does not recompute it or verify the selected probability method. Successful validation
therefore does not establish these physical relationships.

## Public-surface and artifact evidence

`bindings/python/tests/test_cdm.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` cover Python construction, nested setters, KVN/XML parsing
and generation, generic identity, and shared limits. `package-python` and `package-rust` are the
shared built-artifact gates.

## Object identity and ordering

CDM table 3-3 requires both objects to use the same `REF_FRAME`; section 4.3.7.2 orders
`OBJECT1` before `OBJECT2`. Root validation checks both conditions, not merely that two different
object tags exist. Section 5.2.9 restricts `ORIGINATOR`, `MESSAGE_ID`, `OBJECT_DESIGNATOR`,
`CATALOG_NAME`, and `INTERNATIONAL_DESIGNATOR` to ASCII letters/digits, `_`, and `-`; other
free-text fields retain their own domains. `cdm::model` mutates each identifier and the second
object's frame and order, checking the root and generation gates. Python construction requires
explicit covariance provenance and maneuverability rather than fabricating either required fact.

## Verification outcome

CDM 1.0 meets the common [verification criteria](README.md#verification-criteria) for
self-contained format and model validation on Rust and Python. The requirement inventory,
family tests, shared public-route tests, and packaged tests are complementary evidence;
none alone establishes conformance or exhaustive mutation coverage.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).
