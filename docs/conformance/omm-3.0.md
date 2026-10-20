# OMM 3.0 and 2.0 conformance

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

The `omm` suite establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and structural strictness | The shared ODM assignment scanner rejects non-ASCII/control input, overlong lines, malformed assignments, unknown and duplicate keywords, fixed-order violations, and comments outside the beginning of a logical block. OMM registers only its standard-derived keyword ranks and choice/repetition transitions. Members of a keyword choice share a rank, so the scanner compares keys as well as ranks: the other alternative may follow, a repeat of the same keyword may not. |
| XML structure | The OMM sequence registration rejects unknown, duplicate, and reordered children throughout the root, header, metadata, data, mean-elements, spacecraft, TLE, covariance, and user-defined structures. It rejects non-schema attributes while allowing only the fixed unit and user-defined parameter attributes at their applicable leaves. |
| Valid input and preservation | All three shipped KVN fixtures and the shipped XML fixture parse through the public strict API. Generated KVN and XML reparse to the same typed model. |
| XML generation | XML generated from every shipped fixture validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| Numeric values in every block | Validation rejects non-finite values in the mean-elements, spacecraft, TLE, and covariance blocks before either notation is generated, and restates the `inclinationType` and `elementSetNoType` ranges that the typed wrappers enforce only in their constructors. Schema range facets are comparisons, which NaN passes, so finiteness is checked in its own right. |
| KVN number spelling | Every OMM floating-point field — mean elements, spacecraft parameters, TLE parameters, and the shared covariance block — uses the shared ODM number writer. It writes at most 16 significant decimal digits, rounding values whose shortest round-trip spelling needs 17 digits: `0.30000000000000004` becomes `3.0e-1`, and `1.2345678901234567` becomes `1.234567890123457e0`. This can change the stored `f64` when reparsed; it is not lossless binary-value preservation. Values whose rounded spelling would overflow `f64` are rejected before generation. `omm_kvn_generation_spells_numbers_as_ccsds_numbers` and the shared `OdmFloat` tests cover rounding and representability. |
| Shared surface contract | `library::parsing` and `library::output`, `conformance::generation`, the Python API tests, and family Criterion matrices provide common parsing, generation, diagnostics, dispatch, and workload evidence linked from the [conformance review index](README.md). |

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

The item inventory exposes no omitted OMM keyword or logical block. OMM has no exception to the
shared policy on [externally governed values](../design/validation-contract.md#registry-governed-values).

## XML TLE choices

The OMM 3.0 schema requires exactly one of `BSTAR`/`BTERM` and exactly one of
`MEAN_MOTION_DDOT`/`AGOM` when `tleParameters` is present; neither schema choice has
`minOccurs="0"`. KVN and model validation permit these optional book fields to be omitted.
XML parsing and generation enforce both choices and refuse an incomplete XML representation
before streaming output. No missing quantity is synthesized.
`generation::xml_tle_choices_are_required_without_rejecting_kvn_omissions` covers this
notation-specific boundary and schema-valid complete XML.

## Comment normalization

XML has separate outer-data and mean-elements comment lists; flattened KVN has one leading
mean-elements comment position. KVN generation writes outer-data comments followed by
mean-elements comments. KVN parsing stores the combined list in `OmmData.comment` and leaves
`MeanElements.comment` empty. Text and relative order in the combined list survive, but the
original XML ownership cannot be reconstructed.

Shared `UserDefined` blocks retain comments and parameters in separate lists. XML permits
comments interleaved between parameters; parsing keeps each list's order, and both XML and
KVN generation put all comments before the parameters. Comments are not associated with
individual parameters. `conversion::comment_merges_and_user_parameter_interleaving_are_canonicalized`
covers both normalizations. Applications needing the original interleaving must retain the
source document; the library does not provide a second archival representation.

## Allocation and packaged-surface evidence

`fixed_family_allocations` records strict KVN parse budgets of at most 128 allocations/24,000
bytes and preflighted streaming-generation budgets of at most 48 allocations/2,000 bytes for the
shipped OMM fixture. `test_omm.py`, `test_api_consistency.py`, and the shared Python
options matrix exercise typed parsing, both notations, files, TLE conversion, diagnostics, and
limits.
Strict binding audit, generated-stub/doc checks, the full wheel test, and the publishable Rust
artifact check are the reproducible packaged gates.

## Verification outcome

OMM 3.0 and 2.0 meet the same [verification criteria](README.md#verification-criteria)
on Rust and Python: complete feature inventory, strict core validation, explicit
representation limits, reviewed public routes, and packaged-artifact tests. This is not
exhaustive mutation of every editable value. Root validation restates numeric ranges such
as `ELEMENT_SET_NO` after mutation; the inherent Rust validator, TLE writer, and ordinary
generation all delegate to that same gate.

## Edition 2.0 reconciliation

The edition-specific authority is `data/odm_502x0b2s.pdf` and the official
`ndmxml-2.0.0-omm-2.0.xsd` with its common schema. ODM 3.0 annex J identifies the header
and propagator-coefficient changes. Direct schema comparison additionally establishes
which formerly required TLE fields became optional:

| Difference | Decision and evidence |
| --- | --- |
| No `CLASSIFICATION` or `MESSAGE_ID` in the 2.0 header | Both parse notations and mutated models reject these fields; `conformance::odm2::opm_and_omm_2_reject_new_header_fields_in_both_notations`. |
| A present 2.0 TLE block requires `NORAD_CAT_ID`, `ELEMENT_SET_NO`, `REV_AT_EPOCH`, `BSTAR`, and `MEAN_MOTION_DDOT` in addition to `MEAN_MOTION_DOT` | The root edition gate enforces the complete old shape independently of theory; `conformance::odm2::omm_2_tle_shape_is_checked_after_mutation_and_at_both_input_boundaries`. |
| `BTERM` and `AGOM` were introduced in 3.0 | Reject them in 2.0; do not substitute a coefficient with different physical meaning. The same TLE-shape regression covers both input and output notations. |
| `EPHEMERIS_TYPE` numbering was updated in 3.0 | Both schemas use an integer rather than a closed enumeration. Preserve the supplied integer; do not infer the actual propagator from it or certify the generated ephemeris. |
| Mean elements, spacecraft, covariance, and user-defined blocks retain the same shape | Every shipped OMM KVN fixture, with its newer header removed and edition changed, preserves its model through both notations and passes the official 2.0 XSD; `conformance::odm2::omm_2_generation_is_schema_valid_and_round_trips`. |
| Public adapters retain edition | The Python conformance matrix exercises both editions through every ordinary string, file, and conversion route. |

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).
