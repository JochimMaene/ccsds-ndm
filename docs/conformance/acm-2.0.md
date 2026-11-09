# ACM 2.0 conformance

This inventory records maintainer evidence for standalone ACM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: ACM 2.0.
- Semantic and KVN authority: CCSDS 504.0-B-2, principally sections 5 and 6.
- XML authority: NDM/XML schema set 4.0.0 and ACM schema 2.0.
- Profile: validated standalone parsing, complete logical-block preservation, deterministic
  generation, and bounded notation conversion through the shared Rust generation gate.

## Executable evidence

The `acm` and `acm_kvn_allocations` suites establish:

| Concern | Evidence |
| --- | --- |
| KVN structure | The family-local scanner rejects non-ASCII values outside comments and forbidden controls, malformed/unknown/duplicate/reordered assignments, misplaced comments, assignments after history data, unknown/nested/mismatched blocks, invalid outer-block order/repetition, invalid SENSOR nesting, and trailing content. ACM lines may have arbitrary length under CCSDS 504.0-B-2 section 6.6.2. |
| XML structure | The shared sequence engine covers root/header/body/segment, metadata, all six data block types, nested sensor data, history records, user-defined parameters, and their legal attributes. Unknown, duplicate, reordered, and illegal-attribute content is rejected. |
| Schema-model fidelity | ACM 2.0 covariance identifiers and basis identifiers, AD Euler sequence, sensor covariance count, frequency units, CP vector units, and target-momentum vector units are modeled. Non-schema `COV_CONFIDENCE`, `AD_EPOCH`, and AD `ATTITUDE_TYPE` have no public model fields and are rejected on input; split CP/momentum aliases are also rejected. |
| Fixture preservation | All four shipped KVN fixtures preserve their complete typed model through KVN and XML. Generated XML validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| Generation boundary | Materialized and streaming KVN generation reject non-ASCII text and unrepresentable numbers before writing bytes while preserving ACM's arbitrary line lengths. The shared writer rounds representable values to the CCSDS 16-significant-digit format. Fixed-shape CP, momentum, target-attitude, and attitude-history records are validated. |
| Resource behaviour | Attitude and covariance rows allocate exact numeric capacity. History generation uses allocation-free numeric-record/vector writers. Strictly increasing or decreasing attitude histories need no temporary storage for duplicate checks; mixed-order attitude histories use a temporary time/index vector and sorting to check uniqueness without quadratic work. |
| Reproducible workloads | `acm_kvn_history_scaling` registers 100, 10,000 and 50,000-record parse/generate workloads and `acm_xml_history_scaling` the first two, the sizes every history-carrying family shares. |

## Normative inventory reconciliation

Annex A, section A2.2.3 contains 110 ACM implementation-conformance statement rows:

| ICS rows | Subject | Reconciled implementation evidence |
| --- | --- | --- |
| 1–6 | Header | Strict version-first parsing, common required header fields, leading comments, ordering, and duplicate rejection. |
| 7–29 | Metadata | Metadata `CLASSIFICATION` preserved in model/KVN and refused by XML generation (absent from `acmMetadataType`); typed object/time/frame fields, data-element ordering declarations, required fields, comments, units, and strict XML/KVN order. |
| 30–43 | Attitude state history | Typed attitude/rate choices, fixed `NUMBER_STATES` record shape, epoch/value ordering, legal units, block comments, and history representability. |
| 44–58 | Physical description | Complete typed scalar/vector physical model, CP frame dependency, fixed vector shape/units, schema order, and fixture preservation. Value validation covers `DRAG_COEFF` finiteness, `WET_MASS`/`DRY_MASS` as non-negative masses, and per-component finiteness of the fixed three-element `CP`. |
| 59–68 | Covariance history | Covariance type/basis identifiers, fixed row shape/count, epochs, units, comments, and history representability. |
| 69–82 | Maneuver history | Typed maneuver metadata and records, fixed vector/state shapes, units, comments, ordering, and representability. Value validation covers a non-negative `MAN_DURATION`, the `MAN_END_TIME`/`MAN_DURATION` exclusivity and ordered bounds, the frame requirement when `TARGET_MOMENTUM` is present, and per-component finiteness of every maneuver target choice. |
| 83–105 | Attitude determination and sensors | Typed AD fields, Euler sequence, nested sensor blocks, NUMBER_SENSOR_NOISE_COVARIANCE/covariance dimensions, units, nesting, order, and duplicate rejection. |
| 106–110 | User defined | Block placement, leading comments, names/values, order, and XML/KVN preservation. |

The typed validators enforce required fields and locally decidable record/cardinality,
choice-dependent, frame, and fixed-shape rules. ACM's exception to the shared policy on [externally governed values](../design/validation-contract.md#registry-governed-values) is
sensor-model truth, which no self-contained message rule can establish.

## Public-surface and artifact evidence

`bindings/python/tests/test_acm.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` cover every nested section's setters, typed choices,
KVN/XML parsing and generation, generic identity, and limits. `package-python` and `package-rust`
are the common built-artifact gates.

## Verification outcome

ACM 2.0 meets the common [verification criteria](README.md#verification-criteria) for
self-contained format and model validation on Rust and Python. The requirement inventory,
family tests, shared public-route tests, and packaged tests are complementary evidence;
none alone establishes conformance or exhaustive mutation coverage.

## Decisions and limits

**Declared content and intervals.** ADM table 5-3's optional `ACM_DATA_ELEMENTS` must
name the actual blocks in order, including repeated ATT/COV/MAN entries. Uppercase and lowercase
block names are accepted; unknown, missing and extra entries fail. Supplied `START_TIME` and
`STOP_TIME` must have real calendar/ordinal dates or complete relative numeric tags. Empty,
sign-only, and decimal-point-only present XML/model tags are invalid; an empty optional KVN
assignment means absence under the shared input rule. Zero and negative offsets remain valid. They are compared
when they use the same epoch branch; reversed bounds fail. Relative
`MAN_END_TIME` cannot precede `MAN_BEGIN_TIME`, a local consistency check. Calendar/relative bounds are not
converted or inferred from histories. Declared bounds are never checked against history rows,
including directly comparable tags; that consistency check is the caller’s responsibility
(`acm_declared_bounds_do_not_constrain_history_rows`). The declaration, interval and edited-model regressions
in `acm::generation` and `acm::epochs` cover these boundaries.

`TARGET_ATTITUDE` is dimensionless (ADM table 5-7 and §6.9.3.5); supplied KVN units
are rejected (`target_attitude_rejects_supplied_kvn_units`).

**Seconds and finite offsets.** Tables 5-3 and 5-7 and XSD `timeUnits` require seconds
for leap offsets and maneuver durations/times. Days are rejected rather than reinterpreted;
optional `[s]`/`units="s"` annotations are accepted. `TAIMUTC_AT_TZERO` and
`NEXT_LEAP_TAIMUTC` must be finite even after direct model edits. The lexical maneuver time type
preserves its number spelling; optional fixed seconds annotations normalize to omission.
`acm::generation::acm_seconds_and_time_offsets_are_checked_at_every_boundary` exercises these rules.

**Frames are one-way dependencies.** Tables 5-5 and 5-7 require `CP_REF_FRAME` when
CP exists and `TARGET_MOM_FRAME` when TARGET_MOMENTUM exists. A required frame must contain
nonblank text; empty or whitespace-only XML/model values do not satisfy the dependency.
`conditional_frames_must_be_nonblank_when_their_vectors_are_present` checks these cases on
input and through both generation paths. A frame supplied without
its optional vector is permitted, matching the independently optional schema elements. The
library does not infer a missing frame from other declarations.

**Maneuver purposes and targets.** Table 5-7 marks targets conditional on `MOM_DESAT`,
`ATT_ADJUST` and `SPIN_RATE_ADJUST`. Each supplied canonical purpose requires its matching
target; lowercase aliases follow ADM §6.8.6 and blank/underscore spellings (`MOM DESAT`,
including list members) follow ADM §6.8.8. The purpose text itself is stored as written. Purpose strings remain open, including custom purposes
and comma-delimited lists, so unknown purposes do not acquire invented target requirements.
A target quaternion uses the shared quaternion component and magnitude checks, with the documented
rounded-data tolerance. Multiple purposes may require multiple targets: KVN retains them, while
the bundled XSD's single target choice cannot represent them. XML input/output therefore refuses
multiple target alternatives rather than dropping one. An unused momentum-frame declaration
also occupies that XML choice. `acm::generation::acm_maneuver_targets_follow_purposes_without_closing_custom_values`
covers custom/single/multiple purposes, quaternion validation and zero-byte XML refusal.

**Optional sensor numbering.** ADM §5.3.9.6 says each sensor block includes the sensor
number, while table 5-8, its ICS and XSD `sensorDataType` mark `SENSOR_NUMBER` optional.
The table also says supplied numbers are unique and ascending. The library takes the permissive
reading supported by the table/schema: it accepts omission and preserves sensor order. Numbers that are supplied must be positive, unique
and ascending in that order; omitted numbers are neither filled in nor used to reorder sensors.
This permissive reading avoids inventing mandatory identifiers.
`acm::generation::acm_supplied_sensor_numbers_are_in_ascending_order_and_may_be_omitted` covers it.

**Metadata classification and attitude types.** The book's table 5-3 repeats `CLASSIFICATION` in metadata,
while the bundled `acmMetadataType` has no such element. KVN and the model preserve metadata classification independently of header classification.
XML generation refuses a populated metadata classification rather than dropping or moving it.
ATT histories currently require an orientation primary type (`QUATERNION`, `EULER_ANGLES` or
`DCM`), optionally followed by RATE_TYPE rate states. Annex B4's rate-only primary state types
are not supported in ATT histories, despite their presence in the shared enum/schema vocabulary;
they fail explicitly. Table 5-4 defines `ATT_TYPE` as the type of *attitude* data, always listed
before rate data, with orientation examples only, and selects rate data separately through
`RATE_TYPE`; the library reads B4's rate entries as `RATE_TYPE` values. The XSD's shared
`acmAttitudeType` enumeration does not distinguish the two keywords, so this is a book-based
restriction on schema-valid input. Orekit applies the same split (its ACM `AttitudeElementsType`
has only these three values; rate types are a separate `RateElementsType`).

**History time rules.** ADM §5.3.4.4 forbids duplicate time tags; §5.3.7.5 additionally
requires covariance histories to increase monotonically. Covariance time tags are therefore
strictly increasing; attitude history tags must be unique, but the library does not impose
an additional increasing-order requirement absent an explicit rule. Negative relative times
are allowed, and `0.0` and `-0.0` are the same time. The `acm::epochs` chronology tests cover
both readers, edited models, and zero-output generation failures.
ADM §6.8.5 excludes negative zero. Reading nevertheless accepts signed zero as a deliberate
producer-compatibility choice: its numeric zero value is preserved, and a sign cannot make
an otherwise duplicate time tag unique. This acceptance is not a claim that negative zero
is a normative spelling. Floating-point KVN output canonicalizes either signed zero to `0.0`.
History tags use lexical `Epoch` values independently of their numerical state columns. XML retains the stored tag spelling, including absolute calendar and ordinal tags. KVN
repairs decimal-point aliases and refuses overprecision; neither path rounds tags through `f64`.
Exponent source spelling is expanded exactly at input. A block must use one form (relative or calendar), and comparisons
use `Epoch::cmp_same_branch` without time-scale conversion. This implements ADM §§5.3.4.3–5
and the XSD's string-valued history rows.
`acm::epochs::history_time_tags_are_relative_or_absolute_one_form_per_block` covers absolute
input, output validated against the XSD, mixed-form refusal and covariance chronology.

**Maneuver times are relative.** The general absolute-or-relative wording in ADM §5.3.4.3
conflicts with the more specific table 5-7 descriptions of `MAN_BEGIN_TIME` and
`MAN_END_TIME`, which require seconds relative to `EPOCH_TZERO`. The ACM schema agrees
with that table by declaring both fields as `ndm:relTimeType`. The library follows the
specific table and schema in both notations; absolute maneuver times are rejected.
`acm::epochs::acm_maneuver_times_use_relative_numeric_form` covers this decision.

**No-rate spelling.** ADM table 5-4 lists `NONE` among `RATE_TYPE` examples, while the
bundled schema's `attRateType` does not permit that token. KVN reading accepts
`RATE_TYPE = NONE` or `none` as an alias for omission, following the uppercase/lowercase
rule in ADM §6.8.6; both mean attitude-only records. Mixed-case `None` is rejected.
The typed model stores no rate type, and canonical KVN/XML output omits `RATE_TYPE`.
Unexpected rate columns and unknown rate tokens still fail. XML reading rejects explicit
`NONE` because it is outside the schema enumeration.
`acm::epochs::kvn_none_rate_type_normalizes_to_omission_while_xml_token_is_refused`
records the normalization, canonical output, and refusals.

**Line endings.** ADM §6.6.6 permits CR, LF, CRLF, and LFCR terminators. ACM reading
accepts all four through the shared byte-length-preserving normalization, so error offsets
remain aligned with the original input. The no-rate regression exercises every terminator.

ADM annex J states that ACM was introduced in this edition. There is no ACM 1.0 edition
established by that source, so `CCSDS_ACM_VERS = 1.0` is rejected rather than interpreted
using 2.0 grammar. Sensor-model truth and the physical meaning of supplied frames remain
caller context; fixed shapes, declared counts, and numeric domains remain library checks.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

**Sensor frequency.** The XSD's positive `frequencyType` facet applies to model, KVN, and
XML values under the [shared validation contract](../design/validation-contract.md).
`acm::generation::sensor_frequency_schema_bound_applies_in_every_notation` checks constructors,
edited models, and refusal before output.

**Relative-number spelling.** Maneuver time reading accepts finite producer forms such as
`.5`, `5.`, and exponent notation. XML retains the stored spelling. KVN writes the shared
canonical numeric form with at most 16 significant digits, as for other numeric fields.

## Additional schema and producer decisions

**Attitude-determination method.** ADM annex B5 enumerates allowed `AD_METHOD` values.
The library deliberately accepts additional producer methods in KVN/model form and preserves
those strings as a compatibility extension beyond the book; annex B5 is not an external registry. XML input/output
requires the bundled `adMethodType` enumeration: `EKF`, `TRIAD`, `QUEST`, `BATCH`,
`Q_METHOD`, `FILTER_SMOOTHER`, `OTHER`, or their explicitly enumerated lowercase forms.
Mixed case and extensions fail XML representability; they are not rewritten to `OTHER`.
`acm::generation::attitude_determination_method_has_an_xml_enum_but_open_kvn_text` covers both domains.

**Sensor comments.** Leading comments immediately inside `SENSOR_START` are accepted and
preserved as a KVN producer extension. ADM table 5-8 places standard comments immediately
after `AD_START`, and the bundled `sensorDataType` has no comment element. XML generation
therefore refuses sensor comments rather than moving them to the AD block or dropping them.
`acm::generation::xml_generation_rejects_kvn_only_sensor_comments_before_output` covers the refusal.

**Euler-history angles.** ACM preserves finite Euler-history angles without enforcing
ADM §6.8.1's −360…360-degree bound on history columns. This deliberately follows the
XSD's string-valued `attLine` representation and allows unwrapped history angles to remain
usable without normalization. Callers requiring that book bound must check Euler-history
columns themselves. This exception does not relax the fixed row shape, declared units,
quaternion checks, or the typed scalar angle bounds in APM/AEM. The preservation regression
is `acm::parsing::euler_history_preserves_unwrapped_finite_angles`.

**Producer order and open basis text.** `EULER_ROT_SEQ` may precede `NUMBER_STATES` in ATT
and AD KVN blocks, matching Orekit output; generation writes table order. Euler sequences accept
axis letters in any case in KVN/Python and write uppercase. Mixed case is a deliberate lossless
producer alias; XML still requires the schema's uppercase spelling. Euler histories place
`EULER_ROT_SEQ` before `RATE_TYPE` in XML, matching `acmAttitudeStateType`.
`ATT_BASIS` and `COV_BASIS` retain book-suggested free text in KVN/model form; XML requires the
schema enumeration. AD `COV_TYPE = NONE` is a KVN alias that normalizes to omission, as does `RATE_TYPE = NONE`;
XML reading rejects explicit `NONE` because it is outside the schema enumeration.

**User comments.** ADM §5.3.10.5 permits comments between user parameters. Their order survives
both notations. Empty user-parameter suffixes are rejected, and blank separators inside histories
and after sensor blocks are insignificant. Other block comments retain their beginning-only rule.

**Required identity and maneuver choice.** `OBJECT_NAME` stays required, following table 5-3's
mandatory marker and the XML element despite the table's permissive note. Exactly one of
`MAN_END_TIME` and `MAN_DURATION` is required, adopting the XSD choice over the book's softer
“should” wording.

XML history numbers use the same finite-number check as KVN: nonzero underflow is refused,
while representable subnormal doubles are retained (`xml_history_values_refuse_nonzero_underflow_and_keep_subnormals`).

ADM §6.6.2 gives ACM no fixed line-length limit. Both KVN reading and writing therefore
permit lines longer than 254 characters; other families may impose that output limit. Shared encoding, namespace, epoch, `N/A`, whitespace and categorical rules
are specified in the [validation contract](../design/validation-contract.md).

Relative ATT/COV history tags accept exponent doubles (5.3.4.3), using the same exact
exponent-to-decimal expansion as OCM; no binary floating-point time conversion is involved.
The shared 64-byte epoch-storage limit still applies. EULER_ROT_SEQ on
non-Euler histories is retained without semantic use. Rate-only ATT_TYPEs are rejected in
attitude histories; AD_METHOD remains open in KVN but XML has its schema enumeration.
The acm_g7 fixture corrects the book's SENSOR_USED_ typo to SENSOR_USED.

Optional Rust fields used while building mandatory ACM blocks remain Option so incomplete
editable records can be represented and validated together. Presence is enforced by root
validation; callers must not interpret Option as a statement that the book makes a field optional.
Python AcmAttitudeState derives NUMBER_STATES from the first data row at construction and
on `att_lines` assignment. Every assigned row must have the same nonzero width; failed row or
epoch conversion leaves both the rows and count unchanged. An absent rate_type means attitude-only records. AcmCovarianceMatrix permits omitted cov_basis/cov_ref_frame.

Known KVN basis/method aliases canonicalize: for example `FILTER  SMOOTHER` becomes
`FILTER_SMOOTHER`. Unknown open text remains unchanged. OBJECT_NAME is required by the
XML schema; OBJECT_DESIGNATOR remains optional. Exponent-relative tags are covered through
both public readers by `exponent_history_tags_expand_exactly_on_both_public_readers`.

## Additional caller boundaries

ACM uses the shared KVN comment writer: multiline XML/model comments become separate
`COMMENT` records in their original block and order. XML regeneration keeps the original
comment entries; a KVN hop normalizes their grouping. Non-ASCII comment text still causes
KVN generation to fail before output. `multiline_comments_use_shared_normalization_in_every_acm_block`
covers all comment positions, including interleaved user-defined comments.

An explicitly present empty `SENSOR_NOISE_STDDEV` is valid XML when
`NUMBER_SENSOR_NOISE_COVARIANCE` is absent (`sensorNoiseType` extends `xsd:string`, ADM
table 5-8). KVN generation refuses it rather than emitting a blank value that reads as absent.
`empty_sensor_noise_is_xml_only_and_kvn_refuses_before_output` covers schema-valid XML
preservation and zero-byte KVN refusal.

Frame dependencies (`CP_REF_FRAME` for `CP`, `TARGET_MOM_FRAME` for `TARGET_MOMENTUM`)
remain enforced on read and write even though the XSD marks each leaf optional: the book's
conditional requirements determine how those vectors are interpreted. No frame is guessed.
`MAN_END_TIME >= MAN_BEGIN_TIME` is an additional local consistency rule rather than a
separate table 5-7 mandatory marker; the library rejects a negative maneuver interval.

History and scalar validation do not prove DCM orthonormality, positive-semidefinite covariance
or sensor-noise matrices, positive moments of inertia, or `DRY_MASS <= WET_MASS`. Finite
`DRAG_COEFF` may be negative. These physical-consistency checks remain with the caller.
AD `NUMBER_STATES`, when present, must be positive and at least six if `RATE_STATES` is
supplied. It is not checked against an exact sum of attitude/rate widths as in ATT histories. `RATE_TYPE=NONE` is the
documented no-rate alias; `RATE_STATES=NONE` is not a recognized keyword.

An empty PHYS or SENSOR block is accepted where its optional fields permit emptiness.
ADM table 5-9 marks USER_DEFINED mandatory, while XSD `userDefinedType` admits an empty `<user/>`.
Reading follows the schema: empty or comment-only USER blocks are accepted in KVN and XML.
Both generation paths refuse them before writing any bytes.
Repeated user-parameter names preserve their order; empty values and text such as `[unit]`
inside a user value are literal user content. Tabs in numeric histories are accepted as blanks.
Free-text fields retain trailing bracketed text rather than treating it as a unit annotation.
Absolute history tags compare by date/time value across calendar and ordinal spellings,
including different fractional precision; no time-scale conversion occurs.

## Pre-release corrections and producer boundaries

ATT/COV numeric time tags retain exact decimal values. Exponent input expands exactly to plain
decimal at parsing, so its source spelling is not retained. KVN generation adds a digit beside a
bare decimal point (`.5` → `0.5`, `5.` → `5.0`) and refuses tags exceeding the ADM §6.8.4.1
16-digit limit or the integer range, before writing bytes. It never rounds distinct time tags
together. XML preserves the stored decimal spelling. See
`pre_release::acm_history_output_repairs_decimal_aliases_but_never_rounds_time_tags` and
`acm::generation::overprecision_history_time_tags_are_xml_only`.

`pre_release::acm_metadata_classification_is_preserved_in_kvn_and_refused_in_xml` covers the
book/schema classification conflict. `pre_release::empty_user_blocks_are_read_but_never_generated`
covers empty and comment-only USER blocks.

**Known producer files rejected.** Orekit's `data/ccsds/adm/acm/ACMExample03.txt` (CP without
`CP_REF_FRAME`, table 5-5) and `ACMExample09.txt` (TARGET_MOMENTUM without `TARGET_MOM_FRAME`,
table 5-7) are rejected; missing frames cannot be inferred. Orekit also writes AD
`NUMBER_STATES = -1` when unset, which is rejected (XSD `positiveInteger`; a state count cannot be negative).
`acm::parsing::known_orekit_samples_with_missing_conditional_values_are_rejected` covers all three.
