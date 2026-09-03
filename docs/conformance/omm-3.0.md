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
| KVN lexical and structural strictness | The shared ODM assignment scanner rejects non-ASCII/control input, overlong lines, malformed assignments, unknown and duplicate keywords, fixed-order violations, and comments outside the beginning of a logical block. OMM registers only its standard-derived keyword ranks and choice/repetition transitions. Members of a keyword choice share a rank, so the scanner compares keys as well as ranks: the other alternative may follow, a repeat of the same keyword may not. |
| XML structure | The OMM sequence registration rejects unknown, duplicate, and reordered children throughout the root, header, metadata, data, mean-elements, spacecraft, TLE, covariance, and user-defined structures. It rejects non-schema attributes while allowing only the fixed unit and user-defined parameter attributes at their applicable leaves. |
| Valid input and preservation | All three shipped KVN fixtures and the shipped XML fixture parse through the public strict API. Generated KVN and XML reparse to the same typed model. |
| XML generation | XML generated from every shipped fixture validates against the official 4.0.0 master schema. |
| Numeric values in every block | Validation rejects non-finite values in the mean-elements, spacecraft, TLE, and covariance blocks before either notation is generated, and restates the `inclinationType` range that the typed wrapper enforces only in its constructor. Schema range facets are comparisons, which NaN passes, so finiteness is checked in its own right. |
| KVN number spelling | The shared ODM state-vector and covariance writers emit ODM 7.7.1 numbers, so generated KVN reparses instead of carrying `Display` spellings such as `0.30000000000000004`. |
| Shared resource and surface contract | `family_contract`, `family_generation_evidence`, the Python options tests, and family Criterion matrices provide the common bounded parsing/generation, diagnostics, dispatch, and workload evidence linked from `family-shared-contract.md`. |

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
shipped OMM fixture. `test_omm.py` and the shared Python options
matrix exercise typed parsing, both notations, files, TLE conversion, diagnostics, and limits.
Strict binding audit, generated-stub/doc checks, the full wheel test, and the publishable Rust
artifact check are the reproducible packaged gates.

## Verification outcome

The complete ICS inventory, strict core behavior, Python delegation, allocation budgets, and
packaged artifacts have received message-level review. OMM 3.0 is verified on the Rust and Python
surfaces. OMM 2.0 remains available: explicit edition conversion is tested, but it
has not received this complete edition-specific review.
