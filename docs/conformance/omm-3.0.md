# OMM 3.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone OMM 3.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: OMM 3.0.
- Semantic and KVN authority: CCSDS 502.0-B-3 plus editorial corrigendum 1, principally sections
  4 and 7 and the OMM implementation-conformance statement in annex A.
- XML authority: NDM/XML schema set 4.0.0 and OMM schema 3.0.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and notation conversion through the shared Rust generation gate.

## Executable evidence

`just conformance-omm` runs `omm_conformance`, which establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and structural strictness | The shared ODM assignment scanner rejects non-ASCII/control input, overlong lines, malformed assignments, unknown and duplicate keywords, fixed-order violations, and comments outside the beginning of a logical block. OMM registers only its standard-derived keyword ranks and choice/repetition transitions. |
| XML structure | The OMM sequence registration rejects unknown, duplicate, and reordered children throughout the root, header, metadata, data, mean-elements, spacecraft, TLE, covariance, and user-defined structures. It rejects non-schema attributes while allowing only the fixed unit and user-defined parameter attributes at their applicable leaves. |
| Valid input and preservation | All three shipped KVN fixtures and the shipped XML fixture parse through the public strict API. Generated KVN and XML reparse to the same typed model. |
| XML generation | XML generated from every shipped fixture validates against the official 4.0.0 master schema. |
| Shared resource and surface contract | `family_contract`, `family_generation_evidence`, the Python options tests, CLI dispatch test, and family Criterion matrices provide the common bounded parsing/generation, diagnostics, dispatch, and workload evidence linked from `family-shared-contract.md`. |

The existing OMM unit suite separately covers the mean-elements choice, TLE theory-dependent
requirements and mutually exclusive choices, numeric constraints, reference-frame epochs, TLE
conversion, and optional structures.

## Complete ICS reconciliation

The normative implementation-conformance list in CCSDS 502.0-B-3 editorial corrigendum 1,
annex A2.5.2, has 70 numbered items. The repository copy
`data/odm_502x0b3e1.pdf` was reconciled as follows:

| ICS items | Logical requirement set | Reconciled implementation/evidence |
| --- | --- | --- |
| 1-7 | Header, version, comments, classification, creation, originator, ID | `Omm`, `OdmHeader`, root/version validation, strict root scanners, all-fixture tests |
| 8-16 | Metadata and conditional reference-frame epoch | `OmmMetadata`, calendar-epoch tests, required-field and theory tests |
| 17-27 | Required mean-elements block and semi-major-axis/mean-motion choice | `MeanElementsType`, strict choice scanner, choice/numeric/unit tests |
| 28-34 | Optional spacecraft parameters | `SpacecraftParameters`, complete optional-field roundtrips |
| 35-44 | Optional TLE parameters and theory-dependent coefficient choices | `TleParameters`, SGP/SGP4/XP/PPT3 and mutual-exclusion tests, TLE line conversion tests |
| 45-68 | Optional 6x6 position/velocity covariance | `OpmCovarianceMatrix`, complete 21-element roundtrips and strict sequence registration |
| 69-70 | User-defined logical block | typed user-defined parameters, strict keyword scanner and roundtrips |

The item inventory exposes no omitted OMM keyword or logical block. Values delegated by the book
to external SANA registries remain open strings; the library validates locally decidable syntax
and message semantics without embedding a mutable registry snapshot.

## Allocation and packaged-surface evidence

`fixed_family_allocations` records strict KVN parse budgets of at most 128 allocations/24,000
bytes and preflighted streaming-generation budgets of at most 48 allocations/2,000 bytes for the
shipped OMM fixture. `family_surface_cli` validates and converts OMM through the installed binary,
including a zero-document-byte output-limit failure. `test_omm.py` and the shared Python options
matrix exercise typed parsing, both notations, files, TLE conversion, diagnostics, and limits.
Strict binding audit, generated-stub/doc checks, the full wheel test, and the publishable Rust
artifact check are the reproducible packaged gates.

## Remaining verification work

OMM remains available rather than verified. The complete ICS feature inventory and current
packaged surfaces are reconciled; final message-level review remains. Shared delegation is evidence
for that review, not an automatic verified claim.
