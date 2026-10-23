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
| KVN lexical and structural strictness | The shared ODM assignment scanner rejects non-ASCII/control input, malformed assignments, unknown and duplicate keywords, fixed-order violations, and comments outside the beginning of a logical block. OMM registers only its standard-derived keyword ranks and choice/repetition transitions. Members of a keyword choice share a rank, so the scanner compares keys as well as ranks: the other alternative may follow, a repeat of the same keyword may not. |
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

## Theory-dependent TLE requirements

`MEAN_ELEMENT_THEORY` is compared case-insensitively after trimming. Table 4-3 of each edition
makes these keywords conditional; KVN and model validation apply the table of the message's
own edition, and both editions reject `BSTAR` with `BTERM` and `MEAN_MOTION_DDOT` with `AGOM`.

| Theory | TLE block | ODM 3.0 (502.0-B-3 table 4-3) | ODM 2.0 (502.0-B-2 table 4-3) |
| --- | --- | --- | --- |
| `SGP` | required | `MEAN_MOTION`, `MEAN_MOTION_DOT`, `MEAN_MOTION_DDOT` | `MEAN_MOTION`, `NORAD_CAT_ID`, `BSTAR`, `MEAN_MOTION_DOT`, `MEAN_MOTION_DDOT` |
| `SGP4` | required | `MEAN_MOTION`, `BSTAR` | `MEAN_MOTION`, `NORAD_CAT_ID`, `BSTAR` |
| `SGP/SGP4` | required | `MEAN_MOTION` | `MEAN_MOTION`, `NORAD_CAT_ID`, `BSTAR` |
| `PPT3` | 3.0 only | `MEAN_MOTION_DOT`, `MEAN_MOTION_DDOT` | none: not a 2.0 theory |
| `SGP4-XP` | 3.0 only | `BTERM`, `AGOM` | none: not a 2.0 theory |
| other | optional | none | none |

`MEAN_MOTION` comes from 4.2.4.6 (a TLE-based OMM uses it instead of `SEMI_MAJOR_AXIS`).
The 2.0 table marks every TLE keyword "No" and states each condition in its description
("only required if MEAN_ELEMENT_THEORY=SGP/SGP4"), so those sentences are the rule. The 3.0
table instead has an M/O/C column: it marks `NORAD_CAT_ID` `O` although its description keeps
the 2.0 sentence. That is a conflict inside the book, so 3.0 does not require `NORAD_CAT_ID`
(unclear, so lenient). `MEAN_MOTION_DOT` is optional in the model (`Option`) outside these
rows. `omm::parsing::recognized_theory_spellings_keep_conditional_requirements` and
`conformance::odm2::omm_2_applies_its_own_theory_table` cover both editions.

## XML TLE choices

Both OMM schemas require `MEAN_MOTION_DOT` in every `tleParameters`. The 3.0 schema also
requires exactly one of `BSTAR`/`BTERM` and exactly one of `MEAN_MOTION_DDOT`/`AGOM`; neither
schema choice has `minOccurs="0"`. KVN and model validation follow the book and permit these
fields to be omitted outside the theory table above. XML parsing and generation enforce the
schema shape and refuse an incomplete XML representation before streaming output. No missing
quantity is synthesized.
`generation::xml_tle_choices_are_required_without_rejecting_kvn_omissions` covers this
notation-specific boundary and schema-valid complete XML.

## Comment normalization

XML has separate outer-data and mean-elements comment lists; flattened KVN has one leading
mean-elements comment position. KVN generation writes outer-data comments followed by
mean-elements comments. KVN parsing stores the combined list in `OmmData.comment` and leaves
`MeanElements.comment` empty. Text and relative order in the combined list survive, but the
original XML ownership cannot be reconstructed.

Shared `UserDefined.entries` retains XML comments and parameters in their original order,
including interleaving. XML regeneration keeps that order. KVN generation refuses interleaving
because its logical block permits leading comments only.
`conversion::comment_merges_and_user_parameter_interleaving_are_preserved` covers both behaviors.

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
| A present 2.0 XML TLE block requires `NORAD_CAT_ID`, `ELEMENT_SET_NO`, `REV_AT_EPOCH`, `BSTAR`, `MEAN_MOTION_DOT`, and `MEAN_MOTION_DDOT` | The XML edition gate enforces the complete old shape independently of theory; KVN and model validation apply the 2.0 book's own theory conditions ([table](#theory-dependent-tle-requirements)), not the 3.0 ones. `conformance::odm2::omm_2_tle_schema_shape_is_xml_only`, `conformance::odm2::omm_2_applies_its_own_theory_table`. |
| 2.0 `angleRange` is [-180, 360) | The 2.0 common schema narrows `RA_OF_ASC_NODE`, `ARG_OF_PERICENTER`, and `MEAN_ANOMALY` from 3.0's [-360, 360). Like every XSD facet it applies to 2.0 in all notations, so a valid 2.0 model always converts. `conformance::odm2::odm_2_angles_use_the_2_0_range`. |
| `BTERM` and `AGOM` were introduced in 3.0 | Reject them in 2.0; do not substitute a coefficient with different physical meaning. The same TLE-shape regression covers both input and output notations. |
| `EPHEMERIS_TYPE` numbering was updated in 3.0 | Both schemas use an integer rather than a closed enumeration. Preserve the supplied integer; do not infer the actual propagator from it or certify the generated ephemeris. |
| A present XML spacecraft block requires all five spacecraft fields | The 2.0 common schema makes them required while 3.0 makes them optional. Partial blocks remain usable in KVN/model form, and fail only 2.0 XML input/output. `conformance::odm2::odm_2_partial_spacecraft_is_kvn_valid_but_not_xml_representable`. |
| Mean elements, covariance, and user-defined blocks retain the same shape | Every shipped OMM KVN fixture, with its newer header removed and edition changed, preserves its model through both notations and passes the official 2.0 XSD; `conformance::odm2::omm_2_generation_is_schema_valid_and_round_trips`. |
| Public adapters retain edition | The Python conformance matrix exercises both editions through every ordinary string, file, and conversion route. |

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

## Reading, preservation and XML facets

All four CCSDS KVN line terminators are accepted. Numerical input accepts unambiguous producer
spellings such as `1e3`, `.5`, and `1.` as well as more than sixteen digits; nonfinite values,
overflow, and nonzero values below the double range are rejected. KVN output retains the
sixteen-significant-digit limit. GM must be positive (`gmType`) in every notation and in the
model, under the shared [facet rule](../design/validation-contract.md#authority-and-representability);
the book states no range. `NORAD_CAT_ID` is stored as `u32`, so ten-digit values up to
4,294,967,295 are read although table 4-3 says "up to nine digits"; KVN still writes them only
within the shared [integer limit](../design/validation-contract.md#integer-storage-limits).

Empty mandatory XML strings (`<OBJECT_ID/>`) are rejected as empty values, book over XSD, per
the [shared rule](../design/validation-contract.md#authority-and-representability).

**KVN unit aliases.** Besides the table 4-3 spellings, KVN reading accepts the uppercase
spellings the XSD also enumerates: `[KM**3/S**2]` for `GM` and `[REV/DAY]`, `[REV/DAY**2]`,
`[REV/DAY**3]` for `MEAN_MOTION`, `MEAN_MOTION_DOT`, `MEAN_MOTION_DDOT`. KVN output always writes
the table spelling (7.7.1). The model keeps the GM spelling, so XML→XML preserves uppercase
`KM**3/S**2`; the mean-motion units have a single model variant and are written lowercase in
both notations, even XML→XML. Other spellings are rejected.

`UserDefined.entries` preserves XML comment/parameter interleaving exactly. KVN conversion
refuses comments after parameters, because its logical block permits leading comments only.
Parameter names are exact suffixes and must be printable uppercase ASCII without whitespace or
`=` when writing KVN; XML leaves them as strings. An initial `USER_DEFINED_` within the stored
suffix is literal and is not stripped. XML mean-elements comments still merge into data comments
across a KVN hop, in data-then-mean-elements order; XML-to-XML retains both positions.

An XML `userDefinedParameters` block with no parameters may be retained and regenerated as XML.
KVN generation refuses that empty or comment-only block: without delimiters, KVN cannot preserve
its presence or distinguish those comments from the previous logical block.

## TLE export and recognized theories

`to_tle_lines` requires Earth-centered TEME elements and UTC epochs (ODM §4.2.4.6).
It refuses other centers/frames rather than transforming or relabeling orbital elements.
General OMM parsing and generation retain their open center/frame domains. This is an explicit
producer-compatibility exception to the book's TLE-based OMM profile: parsing preserves the
producer's declarations for inspection, and model validation does not certify a propagator/frame
combination. The Earth/TEME/UTC requirement is enforced when producing a TLE. Cyclic TLE
angles are rounded to four decimals and a rounded 360 degrees is emitted as zero;
`omm::conversion::tle_export_requires_earth_teme_and_rounds_cyclic_angle_boundaries`
covers both boundaries and rereads the produced TLE.

Known theories are recognized case-insensitively without changing their stored text and
apply the [theory table](#theory-dependent-tle-requirements). In 3.0 the alias `SGP/SGP4`
requires only what SGP and SGP4 share (the TLE block and `MEAN_MOTION`); in 2.0 it is the
book's own spelling and also requires `NORAD_CAT_ID` and `BSTAR`. Unknown theory names remain
open strings; validation does not certify an external propagator.

An empty or comment-only XML `spacecraftParameters` block is retained in XML but refused
by KVN generation. Without a spacecraft value, flattened KVN cannot retain the block or
its comment association. `conformance::generation::empty_spacecraft_blocks_are_xml_only_and_kvn_refuses_before_output`
covers both OPM and OMM, including zero-byte streaming refusal.

OPM/OMM's [elapsed-time profile](../design/validation-contract.md#opm-and-omm-elapsed-time-profile)
is narrower than ODM's complete MET/MRT duration domain.
