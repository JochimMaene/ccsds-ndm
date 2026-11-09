# OMM 3.0 and 2.0 conformance

This inventory records maintainer evidence for standalone OMM 3.0 and 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message editions: OMM 3.0 and 2.0; see the edition reconciliation below.
- Semantic and KVN authority: CCSDS 502.0-B-3 plus editorial corrigendum 1, principally sections
  4 and 7 and the OMM implementation-conformance statement in annex A.
- XML authority: NDM/XML schema set 4.0.0 and OMM schema 3.0.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and notation conversion through the shared Rust generation gate.

## Executable evidence

The `omm` suite establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and structural strictness | The shared ODM assignment scanner rejects non-ASCII outside comments and control input other than TAB, malformed assignments, unknown and duplicate singleton keywords, fixed-order violations, and comments outside the beginning of a logical block. OMM registers only its standard-derived keyword ranks and choice/repetition transitions. Members of a keyword choice share a rank, so the scanner compares keys as well as ranks: the other alternative may follow, a repeat of the same singleton keyword may not. User-parameter names may repeat and retain their ordered values. |
| XML structure | The OMM sequence registration rejects unknown, duplicate, and reordered children throughout the root, header, metadata, data, mean-elements, spacecraft, TLE, covariance, and user-defined structures. It rejects non-schema attributes while allowing only the fixed unit and user-defined parameter attributes at their applicable leaves. |
| Valid input and preservation | All three shipped KVN fixtures and the shipped XML fixture parse through the public strict API. Generated KVN and XML reparse to the same typed model. |
| XML generation | XML generated from every shipped fixture validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| Numeric values in every block | Validation rejects non-finite values in the mean-elements, spacecraft, TLE, and covariance blocks before either notation is generated, and restates the `inclinationType` and `elementSetNoType` ranges that the typed wrappers enforce only in their constructors. XSD bounding facets exclude NaN, but libxml2 can incorrectly accept it; explicit finite-number checks and the shared Xerces oracle cover that boundary. |
| KVN number spelling | Every OMM floating-point field — mean elements, spacecraft parameters, TLE parameters, and the shared covariance block — uses the shared ODM number writer. It writes at most 16 significant decimal digits, rounding values whose shortest round-trip spelling needs 17 digits: `0.30000000000000004` becomes `0.3`, and `1.2345678901234567` becomes `1.234567890123457`. This can change the stored `f64` when reparsed; it is not lossless binary-value preservation. Values whose rounded spelling would overflow `f64` are rejected before generation. `omm_kvn_generation_spells_numbers_as_ccsds_numbers` and the shared `OdmFloat` tests cover rounding and representability. |
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
| `PPT3` | required in 3.0; optional in 2.0 | `MEAN_MOTION_DOT`, `MEAN_MOTION_DDOT` | none: treated as an open theory string |
| `SGP4-XP` | required in 3.0; optional in 2.0 | `BTERM`, `AGOM` | none: treated as an open theory string |
| other | optional | none | none |

`MEAN_ELEMENT_THEORY` remains an open string in both editions. Edition 2.0 preserves
`PPT3` and `SGP4-XP` without applying their 3.0 conditional requirements; its edition gate
still refuses the later `BTERM` and `AGOM` fields.

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
`MeanElements.comment` empty. Relative order is retained, subject to the shared
[comment normalization](../design/validation-contract.md#preservation-and-normalization),
including splitting multiline entries and trimming trailing blanks. Original XML ownership
and multiline grouping cannot be reconstructed.

Shared `UserDefined.entries` retains XML comments and parameters in their original order,
including interleaving. XML regeneration keeps that order. KVN generation refuses interleaving
because its logical block permits leading comments only.
`conversion::comment_merges_and_user_parameter_interleaving_are_preserved` covers both behaviors.

## Allocation and packaged-surface evidence

`fixed_family_allocations` records strict KVN parse budgets of at most 128 allocations/24,000
bytes and preflighted streaming-generation budgets of at most 48 allocations/2,000 bytes for the
shipped OMM fixture. `test_omm.py`, `test_api_consistency.py`, and the shared Python
workflow matrix exercise typed parsing, both notations, files, TLE conversion, diagnostics, and
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
| 2.0 `angleRange` is [-180, 360) | The 2.0 common schema narrows `RA_OF_ASC_NODE`, `ARG_OF_PERICENTER`, and `MEAN_ANOMALY` from 3.0's [-360, 360). Like every XSD facet it applies to 2.0 in all notations, to keep the 2.0 angle domain compatible across notations. `conformance::odm2::odm_2_angles_use_the_2_0_range`. |
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
4,294,967,295 are read although table 4-3 says "up to nine digits". Negative catalog numbers
are rejected despite `xsd:integer`; this is the documented unsigned storage profile. KVN still
writes these fields only within the shared
[integer limit](../design/validation-contract.md#integer-storage-limits).

XML `ELEMENT_SET_NO`, `NORAD_CAT_ID` and `REV_AT_EPOCH` accept signed integer zero
(`-0`, `-000`) under XSD integer value equality, retaining their field-specific bounds.
KVN unsigned integers still reject a leading minus. The shared
`unsigned_xml_integer_zero_uses_xsd_value_equality` regression covers both standalone editions,
combined 3.0 XML, invalid negatives, range limits and model preservation.

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

ODM 3.0 §4.2.4.6 (2.0 §4.2.4.6 is the same apart from its reference) reads: "For operations
in Earth orbit with a TLE-based OMM, some special conventions must be observed":

- "The value associated with the CENTER_NAME keyword shall be 'EARTH'."
- "The value associated with the REF_FRAME keyword shall be 'TEME'."
- "The value associated with the TIME_SYSTEM keyword shall be 'UTC'."
- "The format of the OBJECT_NAME and OBJECT_ID keywords shall be that of the UN Office of
  Outer Space Affairs designator index" (2.0: "of the SPACEWARN bulletin").
- "The MEAN_MOTION keyword must be used instead of SEMI_MAJOR_AXIS."

For `SGP`, `SGP4` and `SGP/SGP4` (case-insensitive), in both editions, parsing, model
validation, both writers and `to_tle_lines` enforce `CENTER_NAME = EARTH`, `REF_FRAME = TEME`,
`TIME_SYSTEM = UTC` and `MEAN_MOTION`. No conversion or relabeling is performed. There is no
producer-compatibility exception. The designator bullet is not enforced: table 4-2 says "there
is no CCSDS-based restriction on the value" of OBJECT_ID, lists `UNKNOWN`, and tells producers
to use `UNKNOWN` for names absent from the index. That conflict is unclear, so the library is
lenient. `pre_release::sgp4_semantics_apply_before_every_generation_route` and
`omm::conversion::tle_export_requires_earth_teme_and_rounds_cyclic_angle_boundaries` cover it.

**MEAN_MOTION for other theories.** Table 4-3 describes SEMI_MAJOR_AXIS/MEAN_MOTION as
"Semi-major axis (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the Keplerian mean
motion", while the XSD choice is unconditional. §4.2.4.6 requires MEAN_MOTION for TLE-based
elements but does not prohibit it elsewhere, and "preferred" is not a "shall". The library
reads this leniently: `PPT3`, `SGP4-XP` and unknown theories may use either alternative.
Unknown theory strings remain open and do not imply that the library certifies a propagator.

Cyclic TLE
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

KVN reading tolerates lines longer than the book's 254-character limit where applicable;
writing enforces that limit, so an accepted overlong record can require shortening before
regeneration. Shared encoding, namespace, epoch, `N/A`, whitespace and categorical rules
are specified in the [validation contract](../design/validation-contract.md).

CelesTrak GP OMM exports may have empty `CREATION_DATE` and
`ORIGINATOR`. These remain rejected under ODM 7.5.1: obtain populated provenance from
the producer or supply known, truthful values before parsing; parsing does not invent them (the TLE helper below is the only documented placeholder path).
Space-Track ODM 2.0 exports wrapped in `<ndm>` are accepted as NDM/XML 1.0 input.
The envelope's legacy constituent set and editions are checked; mixed legacy/current envelopes
are rejected. Output remains NDM/XML 4.0.0 and refuses stored ODM 2.0 constituents.
`pre_release::legacy_envelopes_are_input_only_and_cannot_mix_editions` uses distributed
fixtures to reproduce this boundary without relying on ignored producer files.


TLE parameters are required for `SGP`, `SGP4`, and (3.0) `PPT3`/`SGP4-XP` theories.
TLE line helpers use the stored TLE fields for `MEAN_MOTION_DOT` and
`MEAN_MOTION_DDOT`: these are already ṅ/2 and n̈/6 (ODM 4.2.4.7 note 2), without further scaling.

`to_tle_lines` accepts only `SGP`, `SGP4` and `SGP/SGP4`; `PPT3`, `SGP4-XP` and other theories
are refused. It needs explicitly populated `NORAD_CAT_ID`, `ELEMENT_SET_NO`, `REV_AT_EPOCH`,
`MEAN_MOTION_DOT`, `MEAN_MOTION_DDOT` and `BSTAR`. Omitted `CLASSIFICATION_TYPE` and
`EPHEMERIS_TYPE` are written as their table 4-3 defaults `U` and `0`, as the
[validation contract](../design/validation-contract.md) allows when the target format needs a
value. Those defaults affect export only; they do not fill the model's fields or invent zero
derivatives. Missing fields raise validation errors naming the field, not an instruction to
infer a propagator. The ordinary metadata and mean-elements requirements also continue to
apply. TLE export requires launch years and the final rounded epoch year in 1957–2056; ordinary
OMM retains its wider date domain.

Repeated user-defined names retain every value in order in KVN and XML. Empty suffixes are
rejected; empty user values are literal user content and remain distinct from missing standard
fields. General validation runs edition restrictions before theory-dependent required fields,
so a 2.0 BTERM/AGOM field is reported as forbidden instead of first asking for a 2.0 BSTAR.

## TLE conversion edition and provenance

The TLE helper emits OMM 3.0, including when MESSAGE_ID is supplied. Parsing ordinary OMM
input requires producer provenance. TLE conversion is an explicit exception: unless overridden,
it uses CREATION_DATE equal to the TLE epoch, ORIGINATOR UNKNOWN and OBJECT_NAME UNKNOWN.
These are documented convenience placeholders, not evidence of the original file creation or
producer. Supply known provenance with TleToOmmOptions when it matters.

`pre_release::tle_helper_emits_valid_omm3_with_message_id_and_export_uses_book_defaults`
covers both the helper edition and export defaults; `pre_release::sgp4_semantics_apply_before_every_generation_route`
covers the shared semantic checks.
