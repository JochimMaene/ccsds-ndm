# CDM 1.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone CDM 1.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: CDM 1.0.
- Semantic and KVN authority: CCSDS 508.0-B-1, principally sections 3 and 6 and annex D.
- XML authority: NDM/XML schema set 4.0.0 and CDM schema 1.0.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and loss-rejecting notation conversion through the shared Rust generation gate.

## Executable evidence

`just conformance-cdm` runs `cdm_conformance`, which establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and logical structure | The family-local scanner rejects non-ASCII/control input, overlong records, malformed assignments, unknown and duplicate keywords, fixed-order violations, nonstandard marked blocks, misplaced comments, extra segments, and trailing content. It models the header, relative data, two metadata sections, and each flattened OD, additional-parameter, state-vector, and covariance block in CDM order. |
| XML structure | The shared XML sequence engine is registered for the complete CDM root, header, body, relative-state vector, two segments, metadata, data, OD, additional-parameter, state-vector, and covariance hierarchy. It rejects unknown, duplicate, reordered, and non-schema children and attributes, including the non-XSD `nil` spelling previously accepted by a permissive test. |
| Delimiter-free COMMENT decision | A leading KVN comment run before the first data keyword is normatively retained on the outer data block; no textual delimiter exists from which to infer a split. If the typed model populates the first present nested block's comments, every materialized and streaming KVN generation route rejects the state before output. The shipped XML fixture demonstrates this richer, XML-only association and is therefore intentionally not convertible to KVN. |
| Typed preservation and generation | All three shipped KVN fixtures retain their complete typed model through deterministic KVN and XML generation. The shipped XML fixture retains its model through XML generation. Every generated XML document validates against the official 4.0.0 master schema. |
| KVN fixed-point preflight | CDM's shape is a fixed, bounded set of scalar records. A private generation preflight writes and reparses that bounded representation before caller-visible output, rejecting lexical violations, multiline normalization, comment reassociation, or any other typed-model change. Streaming rejection is proven to write zero bytes. |
| Shared surfaces and limits | `family_contract`, Python option tests, and the shared generation plumbing exercise bounded parsing/generation, structured diagnostics, and Rust-core delegation. No separate CDM semantics are implemented in an adapter. |

## Normative inventory reconciliation

Annex A, section A2.1.5 contains 68 CDM implementation-conformance statement rows. They map to the
wire/model evidence in these groups:

| ICS rows | Subject | Reconciled implementation evidence |
| --- | --- | --- |
| 1–7 | Header | Strict version-first parsing, common header required fields, comment placement, ordering, and duplicate rejection. |
| 8–19 | Relative metadata/data | Typed relative fields and relative state vector, required-field/unit handling, exact XML order, and fixed KVN order. |
| 20–39 | Object metadata, repeated for OBJECT1 and OBJECT2 | Exactly two segments and one of each object designator, typed identity/orbit/covariance-method fields, required values, units, and ordering. |
| 40–58 | Object data, OD, and additional parameters | Complete flattened KVN ordering and nested XML structure, required and optional typed fields, units, and the documented delimiter-free comment decision. |
| 59–62 | State vector | All six required components, fixed units, ordering, and generated-schema validation. |
| 63–68 | Covariance matrix | Required lower-triangular RTN terms, optional drag/SRP/thrust extensions, fixed units, ordering, and generated-schema validation. |

Externally established object identity, probability methodology, and physical truth are not
self-contained message validation. The parser does enforce the complete structural inventory and
the self-contained two-object/cardinality and required-field rules represented by the typed model.

## Public-surface and artifact evidence

`bindings/python/tests/test_cdm.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` cover Python construction, nested setters, KVN/XML parsing
and generation, generic identity, and shared limits. `package-python` and `package-rust` are the
shared built-artifact gates.

## Status

CDM remains `implemented-unverified` pending reviewed creation of exact operation × notation ×
surface cells. Grouped normative and shared artifact evidence is not, by itself, a promotion.
