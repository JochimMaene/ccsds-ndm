# RDM 1.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone RDM 1.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: RDM 1.0.
- Semantic and KVN authority: CCSDS 508.1-B-1, principally sections 3 and 5 and annex C.
- XML authority: NDM/XML schema set 4.0.0 and RDM schema 1.0.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and notation conversion through the shared Rust generation gate.

## Executable evidence

`just conformance-rdm` runs `rdm_conformance`, which establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and structural strictness | The shared assignment scanner rejects non-ASCII/control input, overlong lines, malformed assignments, unknown and duplicate keywords, fixed-order violations, and comments outside the beginning of one of the seven logical data blocks. RDM registers only its standard keyword order and user-defined repetition. |
| XML structure | The shared XML sequence engine is registered for the complete RDM root, header, metadata, data, and seven nested logical-block families. It rejects unknown, duplicate, and reordered children and non-schema attributes. |
| Conversion preservation | The routed-comment parser associates KVN comments with the logical block selected by the following keyword. KVN generation retains required/optional units for atmospheric, impact, state, covariance, spacecraft, and OD measures. The Annex C XML fixture survives XML-to-KVN conversion modulo the KVN COMMENT separator's non-semantic surrounding-whitespace normalization. |
| Unrepresentable XML state | An XML `<data><COMMENT>` cannot be distinguished from the first atmospheric logical-block comment in flattened KVN. All materialized and streaming KVN gates reject that state before output instead of silently moving the comment. |
| Valid input and generation | Both shipped KVN and both shipped XML fixtures preserve their typed model in their source notation; every generated XML document validates against the official 4.0.0 master schema. |
| Shared resource and surface contract | `family_contract`, `family_generation_evidence`, the Python options tests, and family Criterion matrices provide the common bounded parsing/generation, diagnostics, dispatch, and workload evidence linked from `family-shared-contract.md`. |

The existing RDM unit suite separately covers required metadata and atmospheric data, controlled
re-entry/object choices, reference epochs, ground-impact confidence intervals and bounds, state and
covariance requirements, spacecraft/OD data, and relevant numeric constraints.

## Complete ICS reconciliation

The 142 numbered features in CCSDS 508.1-B-1 corrigendum 1 annex A2.2 were reconciled by complete
logical ranges:

| ICS items | Logical requirement set | Reconciled implementation/evidence |
| --- | --- | --- |
| 1-6 | Mandatory header and message ID | `Rdm`, `RdmHeader`, strict required/root tests |
| 7-20 | Object identity, controlled-reentry, center, time and reference epoch metadata | `RdmMetadata`, enums, required-field and epoch tests |
| 21-41 | Optional orbit, propagator, uncertainty, previous/next-message metadata | complete optional metadata roundtrip and strict sequence evidence |
| 42-52 | Mandatory atmospheric-reentry block and optional windows/confidence | `AtmosphericReentryData`, window/order and numeric tests |
| 53-84 | Ground-impact probabilities, location, and three confidence regions | `GroundImpactData`, complete optional/probability/bounds tests |
| 85-93 | Optional state vector | `StateVector`, completeness/reference-frame tests |
| 94-117 | Optional 6x6 covariance | `OpmCovarianceMatrix`, state dependency and complete 21-element roundtrips |
| 118-129 | Optional spacecraft properties | `SpacecraftParameters`, typed units and complete optional fixtures |
| 130-141 | Optional orbit-determination information | `OdParameters`, count/span/percentage tests and roundtrips |
| 142 | User-defined parameters | typed user-defined map, strict scanner and roundtrips |

No annex-A RDM feature is absent. The standard’s SANA-sourced vocabulary remains external and
caller supplied; locally decidable controlled values and numeric ranges are typed and validated.

## Allocation and packaged-surface evidence

`fixed_family_allocations` fixes strict KVN parse budgets at 96 allocations/12,000 bytes and
preflighted streaming generation at 40 allocations/2,000 bytes. `test_rdm.py` and the shared Python
options matrix cover both notations, files, epochs, generic identity, diagnostics, and limits.
Strict binding audit, generated stubs/doc checks, wheel verification, and Rust artifact
verification are the packaged gates.

## Reproducible performance observation

```text
cargo bench --manifest-path ccsds-ndm/Cargo.toml --bench kvn_benches -- 'kvn_message_matrix/(parse|generate)/rdm' --sample-size 20 --measurement-time 2 --warm-up-time 1
cargo bench --manifest-path ccsds-ndm/Cargo.toml --bench xml_benches -- 'xml_message_matrix/(parse|generate)/rdm' --sample-size 20 --measurement-time 2 --warm-up-time 1
```

On 2026-07-18 the observed 95% intervals were 6.551-7.445 us for KVN parse,
2.394-2.822 us for KVN generation, 29.363-43.171 us for XML parse, and 7.320-9.582 us for XML
generation. These are observations, not unexplained release thresholds; the registered family
benchmarks provide the CodSpeed comparison surface for subsequent changes.

## Remaining verification work

RDM remains `implemented-unverified`. Its complete ICS feature inventory and packaged surfaces are
reconciled, but the grouped evidence still requires exact operation/notation/surface cell review
under the conformance policy before promotion.
