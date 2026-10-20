# ACM 2.0 conformance

This inventory records maintainer evidence for standalone ACM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: ACM 2.0.
- Semantic and KVN authority: CCSDS 504.0-B-2, principally sections 5 and 6.
- XML authority: NDM/XML schema set 4.0.0 and ACM schema 2.0.
- Profile: strict standalone parsing, complete logical-block preservation, deterministic
  generation, and bounded notation conversion through the shared Rust generation gate.

## Executable evidence

The `acm` and `acm_kvn_allocations` suites establish:

| Concern | Evidence |
| --- | --- |
| KVN structure | The family-local scanner rejects non-ASCII/control input, malformed/unknown/duplicate/reordered assignments, misplaced comments, assignments after history data, unknown/nested/mismatched blocks, invalid outer-block order/repetition, invalid SENSOR nesting, and trailing content. ACM lines may have arbitrary length under CCSDS 504.0-B-2 section 6.6.2. |
| XML structure | The shared sequence engine covers root/header/body/segment, metadata, all six data block types, nested sensor data, history records, user-defined parameters, and their legal attributes. Unknown, duplicate, reordered, and illegal-attribute content is rejected. |
| Schema-model fidelity | ACM 2.0 covariance identifiers and basis identifiers, AD Euler sequence, sensor covariance count, frequency units, CP vector units, and target-momentum vector units are modeled. Non-schema `COV_CONFIDENCE`, `AD_EPOCH`, split CP/momentum aliases, and AD `ATTITUDE_TYPE` are rejected rather than emitted or silently lost. |
| Fixture preservation | All four shipped KVN fixtures preserve their complete typed model through KVN and XML. Generated XML validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| Generation boundary | Materialized and streaming KVN generation reject non-ASCII text and unrepresentable numbers before writing bytes while preserving ACM's arbitrary line lengths. The shared writer rounds representable values to the CCSDS 16-significant-digit format. Fixed-shape CP, momentum, target-attitude, and attitude-history records are validated. |
| Resource behaviour | Attitude and covariance rows allocate exact numeric capacity. History generation uses allocation-free numeric-record/vector writers. Strictly increasing or decreasing attitude histories need no temporary storage for duplicate checks; mixed-order attitude histories use a temporary time/index vector and sorting to check uniqueness without quadratic work. |
| Reproducible workloads | `acm_kvn_history_scaling` registers 100, 10,000 and 50,000-record parse/generate workloads and `acm_xml_history_scaling` the first two, the sizes every history-carrying family shares. |

## Normative inventory reconciliation

Annex A, section A2.2.3 contains 110 ACM implementation-conformance statement rows:

| ICS rows | Subject | Reconciled implementation evidence |
| --- | --- | --- |
| 1–6 | Header | Strict version-first parsing, common required header fields, leading comments, ordering, and duplicate rejection. |
| 7–29 | Metadata | Typed object/time/frame fields, data-element ordering declarations, required fields, comments, units, and strict XML/KVN order. |
| 30–43 | Attitude state history | Typed attitude/rate choices, fixed `NUMBER_STATES` record shape, epoch/value ordering, legal units, block comments, and history representability. |
| 44–58 | Physical description | Complete typed scalar/vector physical model, CP frame dependency, fixed vector shape/units, schema order, and fixture preservation. Value validation covers `DRAG_COEFF` finiteness, `WET_MASS`/`DRY_MASS` as non-negative masses, and per-component finiteness of the fixed three-element `CP`. |
| 59–68 | Covariance history | Covariance type/basis identifiers, fixed row shape/count, epochs, units, comments, and history representability. |
| 69–82 | Maneuver history | Typed maneuver metadata and records, fixed vector/state shapes, units, comments, ordering, and representability. Value validation covers a non-negative `MAN_DURATION`, the `MAN_END_TIME`/`MAN_DURATION` exclusivity and ordered bounds, the frame requirement when `TARGET_MOMENTUM` is present, and per-component finiteness of every maneuver target choice. |
| 83–105 | Attitude determination and sensors | Typed AD fields, Euler sequence, nested sensor blocks, declared sensor count/covariance dimensions, units, nesting, order, and duplicate rejection. |
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
`STOP_TIME` are compared when they use the same epoch branch; reversed bounds fail. Relative
`MAN_END_TIME` cannot precede `MAN_BEGIN_TIME` (table 5-7). Calendar/relative bounds are not
converted or inferred from histories. The declaration, interval and edited-model regressions
in `acm::generation` and `acm::epochs` cover these boundaries.

**Seconds and finite offsets.** Tables 5-3 and 5-7 and XSD `timeUnits` require seconds
for leap offsets and maneuver durations/times. Days are rejected rather than reinterpreted;
optional `[s]`/`units="s"` annotations are accepted. `TAIMUTC_AT_TZERO` and
`NEXT_LEAP_TAIMUTC` must be finite even after direct model edits. The lexical maneuver time type
preserves its number spelling; optional fixed seconds annotations normalize to omission.
`acm::generation::acm_seconds_and_time_offsets_are_checked_at_every_boundary` exercises these rules.

**Frames are one-way dependencies.** Tables 5-5 and 5-7 require `CP_REF_FRAME` when
CP exists and `TARGET_MOM_FRAME` when TARGET_MOMENTUM exists. A frame supplied without
its optional vector is permitted, matching the independently optional schema elements. The
library does not infer a missing frame from other declarations.

**Maneuver purposes and targets.** Table 5-7 marks targets conditional on `MOM_DESAT`,
`ATT_ADJUST` and `SPIN_RATE_ADJUST`. Each supplied canonical purpose requires its matching
target; lowercase aliases follow ADM §6.8.6. Purpose strings remain open, including custom purposes
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

**Supported profile limits.** The book's table 5-3 repeats `CLASSIFICATION` in metadata,
while the bundled `acmMetadataType` has no such element. The library supports the standard header
classification only; the additional metadata spelling is rejected in both notations. It does not
move a metadata classification into the header.
ATT histories currently require an orientation primary type (`QUATERNION`, `EULER_ANGLES` or
`DCM`), optionally followed by RATE_TYPE rate states. Annex B4's rate-only primary state types
are not supported in ATT histories, despite their presence in the shared enum/schema vocabulary;
they fail explicitly instead of being interpreted as orientation data. This is a library profile
restriction, not a claim that the publication forbids those states.

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
KVN's 16-significant-digit rounding must also preserve timestamp uniqueness and covariance
order: generation refuses a history whose distinct model times collapse to the same output
time. XML can still represent those distinct `f64` times. The
`acm::generation::kvn_rounding_must_not_collapse_distinct_history_times` regression checks
that refusal before KVN output without banning other 17-digit input values.

**Relative history times only.** ADM §5.3.4.3 permits either absolute or relative history
time tags, with one representation throughout a block (§5.3.4.5). The bundled ACM schema
declares both `attLine` and `covLine` as `xsd:string`; it does not limit these tags to
relative numbers. The library's shared typed rows currently support only relative seconds
from `EPOCH_TZERO` in both notations. This is a library model limitation, not a book/schema
conflict. Absolute
history epochs are rejected rather than converted or guessed. This is a supported-profile
limitation, not a claim that the book forbids them.
`acm::epochs::history_epochs_support_relative_numbers_only` fixes that boundary.

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
