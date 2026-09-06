# Validation evidence and rollout

## Status

Baseline revision `e1c3bc4`. This section is the authoritative status; the per-category narrative
below records how each result was obtained and must not be read as a status summary.

**Complete.**

| # | Category | Evidence |
| --- | --- | --- |
| 1 | Shared RDM/CDM blocks | `OdParameters`, `RdmSpacecraftParameters`, `GroundImpactParameters`, routed from both roots |
| 2 | TDM metadata and observations | Both accepted editions; 503.0-B-1 checked directly and agrees with 503.0-B-2 |
| 3 | ADM repeated states and block values | AEM's nine choices, APM's spin branches and Euler/angular-velocity/inertia blocks, ACM physical and maneuver values |
| 5 | OMM/OEM/OPM/CDM residual accounting | Root-mutation accounting per family; `ELEMENT_SET_NO`, CDM relative metadata, `Vec3Double`, RDM covariance closed |
| 6 | Python delegation | Eight routing shapes, one mutation each, all three surfaces agreeing with Rust |
| 7 | Conformance-claim reconciliation | 29 files audited; overclaims, dead references and duplicated policy corrected |

**Complete with recorded non-claims** — category 4 (OCM). Every block validator covers its own
editable values. `TIME_SPAN` is deliberately unvalidated (operands may carry different time
systems and the block supplies no context to compare them); `OEB_MAX/INT/MIN` ordering is
descriptive rather than a `shall`; `OEB_Q*` has no norm rule because `-999` is the tumbling flag.

**Outstanding.**

1. **Assess the XML, parser and writer duplication separately.** Pilot E settled the KVN ordering
   strand for OPM. The XML sequence tables state the same element order again, per parent and
   nested; the KVN parser match arms and the writers state it a third and fourth time. Whether any
   of those can share the declaration is a separate experiment, and Pilot E is not evidence that
   they can.
2. **Extend performance evidence where real workloads warrant it.** There is one reproducible
   Python baseline, on one machine, for one family, with no allocation counts. The Rust suite has
   allocation budgets for the history-bearing families; wall-clock evidence beyond OEM does not
   exist. Extend it where a workload actually matters rather than for coverage.
3. **Retire temporary machinery and state documents once they stop serving active work.** The
   migration-era duplicate of the OPM ordering implementation has already been removed and
   replaced with standard-derived cases. This document is itself in that category: once the
   outstanding items close, the Status section belongs in the conformance set and the pilot
   narrative belongs in history.
4. **Validate packaged artifacts before treating the result as release-ready.** `package-rust` and
   `package-python` both pass on this snapshot: the crate builds and its doctests run from a
   packaged checkout, and the `manylinux_2_34` abi3 wheel installs into a clean interpreter. That
   is a gate passed, not a release decision.

Consolidation pilots A (constrained scalar) and E (OPM KVN ordering) closed positively; B (enum
macro) and D (Python wrapper generation) closed as reasoned negatives; C found a reachability
defect rather than duplication. The standard Pilot E sets is the one to apply to the remaining
strands: remove a concrete maintenance burden, demonstrate preserved behaviour, and keep only the
mechanism that earned its place. Abstraction for its own sake is not the goal.

**Decided, not open.** TDM edition 1.0 (agrees with 2.0); RDM `NOMINAL_IMPACT_ALT`, OCM phase
angles and OCM `DAYS_SINCE_*_OBS` (P3 finiteness, P4 refusal); OCM `MAN_*` numeric columns (Orekit
parity); TDM 1.0 keyword leniency (accepted); OPM `MAN_DELTA_MASS` (resolved per edition, with
citations, in the contract);
assignment-time validation (setters defer to the root).

## Evidence boundary

The first inventory contains 145 field uses of 21 explicitly constrained wrapper types. It is
useful for locating shared rules and containment routes, but it is not a complete validation map:

- exact-reference read-through showed that field-name searches produce both false positives and
  false negatives through same-name fields, parent-validates-child code, and helper functions;
- the model also contains 45 named raw `f64` fields outside those wrappers; and
- TDM has another 45 raw numeric observation variants, in addition to its metadata fields.

Static inventories therefore route review work; public-entry-point mutation establishes coverage.
Each category below must account for wrapper values, direct numeric fields, optional blocks,
repeated elements beyond index zero, cross-field rules, and target-notation representability.

## Verified read-through of the ambiguous wrapper rows

The 53 rows previously labelled B, C, or D were read at their exact references. The old cell label
is retained only to identify the reviewed set.

| Result | Uses | Evidence |
| --- | ---: | --- |
| Constraint fully enforced at P3 | 28 | `SpacecraftParameters` (5), CDM collision/additional parameters (5), OCM pulse durations and SCLK rate (3), ACM sensor frequency (1), OMM mean elements (6), OPM Keplerian elements and maneuver duration (8) |
| Invalid state excluded by the Rust type | 2 | AEM and OEM interpolation degree store `NonZeroU32`; their validators enforce the associated interpolation dependency |
| Not semantically checked | 21 | APM `SpinState` values (8), AEM spin/nutation values (5), ACM mass/duration values (3), RDM spacecraft values (4), and OCM `TIME_SPAN` (1) |
| Partially checked | 1 | OCM `WEIGHTED_RMS` rejected negative finite values but admitted NaN and positive infinity; closed in category 4 |
| Resolved per edition | 1 | OPM `MAN_DELTA_MASS`: prose says negative in both editions, the 2.0 schema is `negativeDouble` and the 3.0 schema is `nonPositiveDouble` with an inline rationale for attitude maneuvers. See the contract for the citations |

The RDM lifetime pilot subsequently closed three previously uncovered `DayIntervalRequired` uses:
`ORBIT_LIFETIME` and its optional start/end window. It demonstrated one reusable predicate, one
containing-block validator, one parent route, exact diagnostics, and pre-write rejection without a
generic traversal framework.

## Cross-field and representability ledger

| Family / rule | Authority | Current state | Classification |
| --- | --- | --- | --- |
| RDM nominal impact requires frame, longitude, and latitude | RDM §3.5.10 | Enforced by `GroundImpactParameters::validate`, routed from `RdmData` | Implemented in category 1 |
| RDM each confidence interval is all-or-none and intervals are numbered consecutively | RDM §§3.5.13–3.5.15 | Enforced by `GroundImpactParameters::validate` | Implemented in category 1 |
| RDM confidence percentages are strictly increasing | RDM §3.5.16 | Enforced by `GroundImpactParameters::validate` | Implemented in category 1 |
| RDM lifetime and re-entry epochs should resolve consistently | RDM §§3.5.8–3.5.9 | Not compared | Advisory `should`; document, do not reject without an explicit policy |
| OCM duty-cycle required fields, window ordering, and pulse period ≥ duration | ODM §§6.2.8.20.6–6.2.8.20.8 | Enforced by `validate_man_duty_cycle` | Keep |
| OCM `TIME_SPAN = STOP_TIME - START_TIME` | ODM table 6-3 | No check; comparison may need time-system context | Missing contextual rule; experiment needed |
| OCM `OEB_MAX` >= `OEB_INT` >= `OEB_MIN` | ODM table 6-5 descriptions | Not compared | Descriptive wording, not a `shall`; do not reject without an explicit policy |
| OCM `OEB_Q*` quaternion norm | ODM table 6-5 | Not checked, deliberately | ODM assigns `-999` as the tumbling-object flag, so no norm rule exists |
| TDM PATH/PATH_1/PATH_2 choice and mode dependency | TDM table 3-1 | Choice and mode dependency are enforced in `TdmMetadata::validate` | Keep |
| TDM path syntax and index range | TDM table 3-1 | `TdmMetadata::validate` re-parses each populated path, so the XML and tuple-field routes no longer bypass `TdmPath::from_str` | Implemented in category 2 |
| TDM path indices refer to participants actually supplied | TDM table 3-1 | Enforced in `TdmMetadata::validate` against the populated `PARTICIPANT_n` fields | Implemented in category 2 |
| TDM SINGLE_DIFF frequency/range requires `RECEIVE_BAND` | TDM table 3-1 | Enforced by `TdmMetadata::validate_for`, which inspects the segment's actual observations | Implemented in category 2 |
| TDM RADEC frame, interpolation degree, and correction flag dependencies | TDM table 3-1 | Enforced in `TdmMetadata::validate` | Keep |
| TDM integration interval, range modulus, Doppler bias, and indexed delays | TDM table 3-1 | Enforced in `TdmMetadata::validate` for every segment | Implemented in category 2 |
| TDM observation numeric domains | TDM §§3.4–3.5 and observation XSD types | Root validation enforces finiteness, `RHUMIDITY`, transmit-frequency, tropospheric-delay, angle, `TEMPERATURE`, `RCS`, and `STEC` domains | Complete |
| AEM attitude choice/type, segment spans, and record ordering | ADM §4.2 | Enforced compositionally | Keep; numeric state ranges remain separate |
| OEM object/time identity, spans, record ordering, and interpolation dependency | ODM §5.2 | Enforced compositionally | Keep |
| OMM mean-element choice and TLE theory-dependent choices | ODM §4.2 | Enforced; the `ELEMENT_SET_NO` range is now restated in `validate_values` | Keep |
| CDM segment identity and covariance optional-row dependencies | CDM §§3.2–3.5 | Enforced; optional OD values and the relative-metadata numbers are now covered too | Keep |
| CDM relative metadata numbers are finite | CDM tables 3-2/3-3 (plain doubles) | Enforced by `RelativeMetadataData::validate` | Implemented in category 5 |
| OCM duty-cycle direction vectors are finite | `vec3Double` list type | Enforced by `validate_man_duty_cycle` | Implemented in category 5 |

Notation and governance conflicts are separate redesign decisions, not ordinary range fixes:

- OCM phase angles and days-since-observation values are broader in ODM than in the XSD.
  **Resolved** with the same P3/P4 split as RDM's altitude: finiteness at P3,
  `Ocm::validate_xml_representability` refuses the conversion at XML generation.
- OPM `MAN_DELTA_MASS`: the authorities genuinely differ — 502.0-B-3 §3.2.4.7 says the value
  "must be a negative number" while the 3.0 schema's `deltamassTypeZ` permits zero with an inline
  rationale for attitude maneuvers. Resolved per edition: zero accepted in 3.0, rejected in 2.0.
  The reasoning and citations are in the contract; it is scoped to this field and these editions
  rather than being a general precedence rule.
- RDM nominal impact altitude has no book range while the XSD embeds an Earth-derived range.
  **Resolved**: the model preserves any finite altitude, P3 enforces finiteness only, KVN writes
  it, and `Rdm::validate_xml_representability` refuses XML generation outside `[-430.5, 8848]`.
  This is the first P4-only rule in the crate and the worked example of the contract's policy for
  book-wider-than-XSD conflicts.
- OCM `OBJECT_TYPE` is governed by a living SANA registry while the XML schema is closed; CDM and
  RDM use closed lists.
- shared `TimeUnits::Day` can create `Duration` values that the XML `timeUnits` type cannot
  represent.
- TDM `RANGE_UNITS` XML deserialization formerly accepted only `km`, `s`, and `RU`; the current
  review tree accepts all six XSD spellings while KVN remains intentionally case-insensitive.
- TDM `ANGLE_1` and `ANGLE_2` use `[-180, 360)` in TDM 2.0 §§3.5.4.2–3.5.4.3 but the shared XSD
  angle type uses `[-360, 360)`; `TEMPERATURE` is positive in §3.5.8.3 but `nonNegativeDouble` in
  the XSD; and `RCS` (§3.5.5.2) and `STEC` (§3.5.7.1) are positive in prose but unconstrained
  doubles in the XSD. Resolved: in each case the book is strictly narrower than the schema, so the
  book's narrower rule is enforced at P3 on its own authority: 503.0-B-2 states the bound
  explicitly and the schema merely fails to express it. (An earlier draft cited OPM
  `MAN_DELTA_MASS` as precedent for "narrower always wins"; that reading was wrong — see the
  contract — and the TDM case does not depend on it.) No book-valid value becomes unrepresentable in XML, so no P4 rule is needed.

For these cases the model must preserve book-valid information, while P4 rejects a target edition
that cannot represent it. Public Rust type changes require compatibility review before selection.

## Ordered implementation plan

### 1. Complete shared RDM/CDM blocks

See the Status section above for this category's standing.

Add compositional validation to `OdParameters` (shared by CDM and RDM),
`RdmSpacecraftParameters`, and `GroundImpactParameters`, then route each populated optional block
from both roots. The implementation covers 37 of the 38 explicitly constrained uses, the remaining
finite numeric values, and the RDM all-or-none and confidence-order rules. The final use,
`NOMINAL_IMPACT_ALT`, remains deliberately isolated until its book/XSD representation decision is
made.

This is the highest-value next category: it reuses the proven RDM slice, covers two families and
optional-block construction, and replaces no working architecture. No implementation experiment is
needed.

### 2. Audit TDM metadata and observation policy as one segment

All metadata and observation rules described below are enforced compositionally
through each segment, including later segments and observations; invalid streaming generation
writes zero bytes. All six XSD `RANGE_UNITS` spellings are accepted. The angle, `TEMPERATURE`,
`RCS`, and `STEC` authority conflicts are resolved in favour of the narrower book range (see the
ledger above); `tdm_observation_domains_follow_the_narrower_book_range` covers the invalid side
including NaN and infinity, `tdm_book_narrowed_boundaries_generate_valid_xml` covers the accepted
boundaries against the XSD oracle, and `test_mutated_observation_value_is_revalidated` covers the
Python observation-list routing shape.

**Legacy edition: resolved against 503.0-B-1.** `versioning.rs` accepts `CCSDS_TDM_VERS` of both
`1.0` and `2.0`, and the observation rules are edition-blind, so the four narrowed domains apply to
parsed TDM 1.0 documents too. 503.0-B-1 was obtained and checked directly; the editions agree, so
no edition-conditional rule is needed:

| Rule | 503.0-B-1 | 503.0-B-2 |
| --- | --- | --- |
| `ANGLE_1`, `ANGLE_2` | "shall be a double precision value as follows: -180.0 <= ANGLE_1 < 360.0" | identical |
| `TEMPERATURE` | "shall be a positive double precision type value" | identical |
| `STEC` | "The value shall be a positive double precision value" | identical |
| `RCS` | keyword does not exist in edition 1.0 | added in 2.0 |

`RCS` being absent from 1.0 makes that rule vacuous for legacy input rather than conflicting. It
does surface a separate, minor edition-strictness question: the observation-keyword scanner is
edition-blind, so a document declaring `CCSDS_TDM_VERS = 1.0` may carry `RCS` or `MAG` and is
accepted. **Decided: left as known leniency.** Accepting a 2.0 keyword in a 1.0 document loses no
information and output is 2.0 only, whereas closing it needs a per-edition keyword inventory —
exactly the kind of independently maintained standards data the project goal aims to reduce. This
is a parse-strictness gap, not a value-domain one.

Policy experiment complete: under `MODE=SINGLE_DIFF`, TDM §§3.3.2.5.5 and 3.3.2.5.7 identify
`RECEIVE_FREQ` and `RANGE` as the differenced-frequency and differenced-range observables. Those
two variants trigger the conditional `RECEIVE_BAND` requirement; indexed receive frequencies,
`DOR`, and `VLBI_DELAY` do not. The observation authority conflicts listed above remain isolated
from the unambiguous metadata and observation rules.

Inventory the 24 named raw metadata doubles and every observation variant against TDM 2.0. Add
notation-neutral numeric checks, validate path references against populated participants, and make
the `SINGLE_DIFF`/`RECEIVE_BAND` rule depend on actual observations. Preserve rules that require an
ICD or external knowledge as caller context.

This category is second because the wrapper-only inventory largely missed TDM. A small policy
experiment is needed to define which observation variants count as differenced frequency/range
before implementation.

### 3. Complete ADM repeated attitude and editable block values

AEM root validation revisits all nine attitude-state
choices, including quaternion normalization, angle bounds, non-negative nutation periods, and
finite rates. APM spin validation covers both optional branches. ACM physical and maneuver
validation covers masses, durations, finite vectors and moments, and every maneuver target choice.
Tests exercise a non-first AEM record, every AEM choice, both APM spin branches, all ACM field
groups, valid boundaries, zero-byte streaming rejection, and representative live Python edits.

Validate APM `SpinState`, every AEM attitude choice, ACM physical masses and maneuver duration, and
the relevant direct numeric fields. Exercise repeated index 1 and every optional choice branch.
Reuse existing choice, quaternion, timeline, and KVN-representability checks; do not replace them.

This category needs a mutation-table prototype for one AEM wrapper because it has the hardest route:
repeated record → mutually exclusive optional branch → constrained values.

### 4. Complete reached-but-partial OCM validators

`OcmPhysicalDescription` now validates every
editable value it owns (areas, masses, percentages, `REFLECTANCE`, the three attitude angles, and
the remaining plain doubles); `OcmPerturbations` covers `GM`, `EQUATORIAL_RADIUS`,
`CENTRAL_BODY_ROTATION`, the geomagnetic and solar-flux values, and the `OBLATE_FLATTENING`
non-finite hole; `OcmOdParameters` covers the OD spans, eigenvalue lengths, `OD_CONFIDENCE`,
`SEDR`, and the `GDOP`/`WEIGHTED_RMS` finite holes; and `TRAJ`/`COV` history lines now reject
non-finite numbers with a line-and-column diagnostic. `Gm::validate_value` was extracted so the
positive-`gmType` rule has one implementation, and `require_finite` replaces the repeated
finite-check block in new code. Tests cover the invalid side (NaN, both infinities, out-of-range),
the accepted boundaries against the XSD oracle, a non-first `trajLine`, and two Python routing
shapes.

Resolved since: `MAN_*` line values stay `Vec<String>` because the columns are heterogeneous, but
every column `MAN_COMPOSITION` declares numeric must hold a finite number, with `ACC_INTERP`,
`THR_INTERP`, and `DEPLOY_ID` as the only non-numeric columns in ODM tables 6-8 and 6-9. No
per-column domain is imposed, which matches Orekit's `ManeuverFieldType` — it parses each
declared-numeric column as a double and enforces no positivity or range, not even for `DEPLOY_MASS`
where the book says "shall be <= 0". The XSD types the maneuver line as a string list, so the
schema oracle accepts text there and only a direct test establishes the rule. The first
implementation broke the recorded maneuver-parse allocation budget by upper-casing a token per
column per line; the column mask is now built once per block.

Metadata `TIME_SPAN` stays unvalidated by decision, not by omission: ODM table 6-3 defines it as
`STOP_TIME - START_TIME`, but the operands may carry different time systems and the block does not
supply the context to compare them, so enforcing it would risk rejecting valid messages.

`Gm::validate_value` is currently called only from OCM. That is a consolidation opportunity, not a
coverage gap: `Opm::validate` (`messages/opm.rs`) and `Omm::validate` (`messages/omm.rs`) each
already reject non-finite and non-positive `GM` through their own inline checks, so the rule has
three correct implementations rather than one. Not calling the shared helper is never by itself
evidence that a value is unvalidated; only reading the root's actual behaviour settles that.

Extend the existing physical, covariance, perturbation, maneuver, and OD validators to their own
editable values; fix the finite-value hole in `WEIGHTED_RMS`; and validate metadata `TIME_SPAN` only
where operands can be compared without inventing time-system context. Preserve the working history
and duty-cycle validation.

Resolve the OCM phase-angle and signed days-since-observation representation conflicts as separate
public-model decisions before changing those fields.

### 5. Close OMM/TLE and residual family gaps

`ELEMENT_SET_NO`, the CDM relative-metadata numbers, and the
`Vec3Double` components are closed; the OPM, OEM, CDM, and fixed-size accounting below is done.
The residual items are listed at the end of this section.

**Method.** Findings in this category come from mutating through the public root and observing
`validate`, `to_kvn`, and `to_xml`, not from searching for helper calls. A throwaway probe test
reported accept/reject per mutation; only confirmed defects were then turned into permanent tests.
This matters: an earlier draft of this document claimed OPM/OMM `GM` was unvalidated because
neither called the new `Gm::validate_value`. Both roots reject non-finite and non-positive `GM`
through their own inline checks. "Does not call the shared helper" is a consolidation observation,
never evidence of a coverage gap.

| Root mutation | Result | Disposition |
| --- | --- | --- |
| OMM `ELEMENT_SET_NO = 100000` | Accepted by `validate`, KVN, and XML; `xmllint` rejected the emitted document with `[facet 'maxInclusive']` | Confirmed defect, fixed: `ElementSetNo::validate_value` extracted and restated in the reached `validate_values`. `elementSetNoType` is `[0, 9999]` in both the 2.0 and 3.0 schemas, so no edition split is needed |
| OMM `REV_AT_EPOCH = u32::MAX` | Accepted | Not a defect; `xsd:nonNegativeInteger` has no upper facet |
| OPM eccentricity `-1`, inclination `190`, `MAN_DURATION -1`, `MAN_DELTA_MASS +1` and NaN, state vector infinity | All rejected at `validate`, KVN, and XML | Confirms the ledger's "Keep" rows by behaviour rather than by inspection |
| OEM state vector NaN at record index 1, covariance `CX_X` NaN | Both rejected | Confirms repeated-record and optional-block reachability |

The CDM and fixed-size accounting was then run the same way, over the populated blocks of
`cdm_363.kvn`, both segments, an optional `CTHR_*` sub-block constructed from scratch, segment
cardinality, OBJECT1/OBJECT2 pairing, and struct-literal constructor bypasses. Two more defects:

| Root mutation | Result | Disposition |
| --- | --- | --- |
| CDM `relative_metadata_data` numbers (`MISS_DISTANCE`, `RELATIVE_SPEED`, the six `relative_state_vector` components, `SCREEN_VOLUME_X/Y/Z`) set non-finite | Accepted by `validate` and XML while the KVN writer rejected them; `xmllint` rejected the emitted document: `'inf' is not a valid value of the atomic type 'xs:double'` | Confirmed defect, fixed in `RelativeMetadataData::validate`. The three output paths had disagreed, which is worse than a uniform gap: the same model generated one notation and not the other |
| OCM `man[i].dc_ref_dir` / `dc_body_trigger` components set non-finite | Accepted by `validate`, KVN, and XML; `xmllint` rejected `'inf 0 0' is not a valid value of the list type 'vec3Double'` | Confirmed defect, fixed in `validate_man_duty_cycle`. `Vec3Double` has three public `f64` fields and a constructor that checks nothing; these two fields are its only uses in the crate |

Confirmed working, by the same method rather than by inspection: CDM `Probability` bounds
(including the `Probability { value: 1.5 }` struct-literal bypass), `NonNegativeDouble`,
`PositiveInteger`, `Percentage`, `DayInterval`, `Mass`, `Area`, `Wkg`, state-vector and covariance
values — all in **both** segments, including fields the fixture omits in segment 1 — plus segment
cardinality, OBJECT1/OBJECT2 pairing, and the optional `CTHR_*` block. Among fixed-size types,
`Vector3`, `TargetMomentum`, and `Vec4Double` reject both wrong lengths and non-finite components;
only `Vec3Double` was unguarded.

Note one deliberate non-finding: CDM `MISS_DISTANCE = -1` is accepted, and that is correct. A
negative miss distance is nonsensical, but neither the schema nor the book constrains the sign, so
inventing the rule would violate the project's own standard of provenance.

Remaining in this category: the CDM header, epoch, string, and enum fields; the individual
`relative_state_vector` components not probed one by one (same types in the same struct as the
probed ones, now covered by the shared block validator); and CDM XML-side parse validation, which
was only exercised root-mutation → emit. Also still open: whether the three correct `GM`
implementations should collapse onto `Gm::validate_value`. That consolidation is secondary to
finding real coverage gaps and must not be scheduled ahead of them.

Existing verified validators should receive tests, not rewrites.

### 6. Verify Python delegation for every completed category

`bindings/python/tests/test_validation_delegation.py` covers eight routing
shapes, one mutation each, editing through the live Python graph and asserting that `validate`,
KVN generation, and XML generation all reject — the same verdict Rust gives:

| Routing shape | Mutation |
| --- | --- |
| Repeated record beyond index 0 | OEM `segments[0].data.state_vector[1].x` |
| Repeated segment → record → choice branch → constrained value | AEM `segments[1].data.attitude_states[1].values[0]`, breaking quaternion normalisation |
| Required scalar on a required block | CDM `body.relative_metadata_data.miss_distance` |
| Fixed-size vector component | ACM `segment.data.phys.cp[1]` |
| Unguarded vector type in an optional block | OCM `segment.data.man[0].dc_ref_dir` |
| Cross-field rule | RDM `impact_1_confidence` populated without its bounds |
| Edition-conditional rule | OPM `MAN_DELTA_MASS = 0`, accepted at 3.0 and rejected once `version` is 2.0 |
| Optional block constructed from scratch | OCM `data.od` built in Python, then `weighted_rms` set to NaN |

This is delegation evidence, not a second copy of the Rust matrix.

**Finding, now fixed: assignment-time validation was inconsistent.** The contract allows invalid
intermediate models and puts enforcement at parsing and generation. About 928 Python setters
followed that; nine did not. Eight in `bindings/python/src/cdm.rs` (the `AdditionalParameters`
constructor and its `area_pc`, `area_drg`, `area_srp`, and `mass` setters) and one in `ocm.rs`
(`GM`) routed through the validating core constructor via `api::checked_optional` and raised on
assignment. The effect was visible for the same wrapper type:
`ocm…phys.wet_mass = -1.0` was accepted and rejected later, while
`cdm…additional_parameters.mass = -5.0` raised immediately.

The nine were aligned to the contract, and `api::checked_optional` was removed as its last caller
went away. Loosening a setter is only safe if the root already rejects the value, so that was
established first: all five fields reject `-5`, NaN, and infinity — plus `0` for the
`positiveDouble` `GM` — through `validate`, KVN, and XML, in both CDM segments.
`TestAssignmentDefersToRootValidation` now pins that as a parametrised matrix rather than pinning
the old divergence.

### 7. Reconcile conformance claims

Both the claim reconciliation and the documentation consolidation were carried out; see the
Status section above.

A read-only audit of all 29 files in `docs/conformance/` produced these corrections, each verified
against source before it was applied:

- **Dead type references in `rdm-1.0.md`.** Three rows named types that are not in the RDM graph:
  `SpacecraftParameters` (the OPM/OMM type; RDM uses `RdmSpacecraftParameters`), `GroundImpactData`
  and `AtmosphericReentryData` (neither exists; the types are `GroundImpactParameters` and
  `AtmosphericReentryParameters`). Corrected, and the rows now state what those validators cover.
- **Fixture-presence overclaims.** `ocm-3.0.md` offered "exercised by the fixture corpus" and
  "fixture round trips" as reconciliation evidence for the Physical, Perturbation, and OD groups —
  120 ICS rows with no editable-value statement. `cdm-1.0.md`, `tdm-2.0.md`, `aem-2.0.md`,
  `acm-2.0.md`, and `apm-2.0.md` had the same shape. All now state what their block validators
  actually cover.
- **`omm-3.0.md` declared the family verified** without recording that post-verification root
  mutation found an accepted `ELEMENT_SET_NO` out of range that emitted schema-invalid XML. The
  outcome section now records the defect and defines what "verified" covers.
- **Deliberate non-claims made explicit.** `ocm-3.0.md` now lists the five OCM values that are not
  validated and why; `rdm-1.0.md` names `NOMINAL_IMPACT_ALT`; `tdm-2.0.md` records the edition-1.0
  exposure as a promotion blocker.
- **`rdm-1.0.md` contradicted the contract** by implying all SANA vocabulary is caller-supplied.
  `OBJECT_TYPE` is a closed list in RDM and unknown values are rejected. Corrected and linked.
- **Schema-validation evidence qualified in eight files.** "Generated XML validates against the
  official 4.0.0 master schema" was repeatedly offered where numeric-value evidence was meant. The
  contract's own XSD oracle policy records that libxml2 accepts NaN against bounding facets, so
  each of the eight claims now says it establishes structure, ordering, and lexical form only.
- **One code fix.** `tests/xsd_schema_validation.rs` silently returned with a printed notice when
  `xmllint` was absent, so the file whose whole purpose is schema validation could pass having
  validated nothing. It now panics like the family evidence tests do.

The documentation consolidation then followed as its own pass:

- **Externally governed values.** The registry/SANA policy was restated in nine places, and the
  copies disagreed — the RDM one was outright wrong. `family-shared-contract.md` now carries one
  "Externally governed values" section covering the SANA-registry rule, the closed-list exception,
  and the mission-truth boundary. Each family keeps only its own exception and links to it: OCM's
  registry-governed `OBJECT_TYPE`, CDM's closed-list `OBJECT_TYPE` and methodology caveat, TDM's
  annex-B frames, ACM's sensor-model truth, APM's frame and time values.
- **Promotion policy.** Seven near-identical paragraphs said the same thing in different words,
  and two families used "available rather than verified" while five used `implemented-unverified`
  for the same state. `family-shared-contract.md` now carries one "Promotion policy" section that
  defines the term, reconciles it with the support matrix's **Available**, and states that grouped
  evidence does not promote a cell. Every family now uses one vocabulary and states only its own
  blocker: AEM's interpolation-degree conflict, TDM's edition-1.0 exposure, combined NDM's G22
  `MASS` conflict, RDM's `NOMINAL_IMPACT_ALT`, OCM's five unvalidated values.
- **Duplicated decisions.** The CDM delimiter-free COMMENT decision and AEM's optional-fixed-unit
  normalisation were described in both the family document and `family-shared-contract.md`. The
  family documents are now the single description — AEM's row gained the 504.0-B-2 §6.9.2/§7.6.10
  citations it was missing — and the shared contract links to them. Same for OCM's `TIME_AND_ANGLE`
  lexical form.
- **OEM/AEM interpolation degree.** Not merged: two families, two fixtures, one decision. OEM's
  vaguer wording now adopts AEM's specificity and cross-links it.

Twenty restatements became two canonical sections plus short family-specific exceptions. Line count
went up, not down: the claim corrections above added substantive evidence statements, and reducing
maintenance obligations was the objective rather than deletion.

## Consolidation pilots

These are the bounded experiments required before any wider consolidation. Each was judged by
whether it removes an *independently maintained fact*, not by lines deleted.

### Pilot A — one constrained scalar: PASSED, and generalised

`nonNegativeDouble` and `positiveDouble` were each implemented independently in many places:
`DayInterval`, `Mass`, `Area`, `NonNegativeDouble`, and `Duration` all carried their own
non-negative check (four byte-identical, one restructured with different wording), and
`DayIntervalRequired`, `Gm`, plus inline blocks in `ocm.rs` (×2), `acm.rs`, and `tdm.rs` each
carried their own positive or non-negative check.

`require_non_negative` and `require_positive` now hold those two rules. **Twelve independent
implementations became two**, with one wording each instead of three. The two failure modes stay
distinct — non-finite reports "not a number the standard can express", out-of-domain reports the
bound — because they mean different things to a caller. Full suite, clippy, and the allocation
budgets are unchanged.

The rule generalises to any XSD type that resolves to `nonNegativeDouble`/`positiveDouble`.

**Where it stops.** The closed-range validators — `Angle` `[-360, 360)`, `Percentage` `[0, 100]`,
`Probability` `[0, 1]`, `LatitudeRequired`, `LongitudeRequired`, `ElementSetNo` — were left alone
deliberately. Their bounds and their upper-bound inclusivity differ per type, so a shared helper
would have to carry both as parameters: the per-type fact survives and only the error-construction
boilerplate collapses. That is line reduction without fact reduction, which the agreed principle
rules out.

### Pilot B — one enum: FAILED as designed, and worth the attempt

The intended pilot was a macro generating `Display`, `FromStr`, and the serde renames from one
variant list, which would remove the current three-way restatement of every wire spelling and
derive the `expected` diagnostic instead of hand-maintaining it.

Auditing the 33 hand-rolled `FromStr` impls first showed why the naive form is wrong. Nine
deliberately accept aliases their `expected` list does not advertise, and the leniency is
load-bearing:

- `DisintegrationType` accepts `MASS-LOSS + BREAKUP`, which no XSD permits, because the published
  example `data/xml/rdm_c4.xml` uses that spelling. Generation always emits the canonical
  `BREAK-UP`, so the non-conforming spelling is normalised rather than propagated.
- `TdmReferenceFrame` accepts `ITRF1993` and `TOD_EARTH` because those are the *book's* spellings,
  while `ITRF-93` and `TOD` are the *XSD's*. Both authorities are legitimate.

A macro deriving `expected` from the accepted spellings would advertise a non-conforming spelling
as valid; one deriving it from canonical spellings only would silently drop the alias arm. The
distinction between "spellings to advertise" and "spellings to accept" is a real per-enum decision
that no mechanical rule reproduces. The macro was therefore not written, and the 22 alias-free
enums are not worth a macro on their own once the 9 must stay hand-written — the value was in
uniformity, and uniformity is unreachable.

The audit paid for itself anyway: `TdmReferenceFrame`'s diagnostic listed only the XSD spellings,
so a reader of the book who mistyped a frame was told to use vocabulary their authority does not
contain. It now lists both. `DisintegrationType`'s alias now records why it exists.

**Two Python binding defects surfaced from the same thread**, both fixed with regression tests:

- `OcmTrajState.__init__` documented and accepted `traj_basis` and `orb_revnum_basis`, then
  silently discarded both, on the false premise that the enums lacked `FromStr`. Caller data
  vanished with no error. `audit_bindings.py` passes on this because the fields *are* exposed —
  the audit checks presence, not that a constructor argument reaches the model.
- Seven enum getters formatted with Rust's `Debug`, so `orb_revnum_basis` read back as `'One'`
  when the wire value is `1`, and assigning a property's own value straight back raised. They now
  use `Display`, i.e. the wire spelling, and round-trip.

### Pilot C — one OPM structural block: already consolidated; the gap was reachability

The intended target, `OpmCovarianceMatrix`, turns out to be a prior consolidation success rather
than a candidate. It lives in `common.rs`, is shared by OPM, OMM, and RDM, and already has one
`kvn_numbers()` table serving both families' representability checks and one `Validate` impl for
its 21 entries. There is no duplicated fact left to remove.

What the survey found instead was a **root-reachability defect**, which the contract's rule 3
names explicitly: "A correct but unreachable child validator is a defect." `RdmData::validate`
checked the covariance/state-vector dependency but never called the shared validator, so RDM
accepted non-finite covariance entries that OPM and OMM both rejected. Confirmed by root mutation:
`cx_x = NaN` passed `validate`, KVN, and XML. The XSD oracle does not catch it — libxml2 accepts
NaN against `xsd:double`, exactly as the contract's oracle policy records — so only a direct test
establishes it. Now routed, with a regression covering NaN, both infinities, and a later entry so
the fix cannot pass on the first element alone.

**The transferable lesson:** consolidating a block into a shared type does not make it reached.
Sharing and routing are independent properties, and the sharing hides the omission, because the
validator's existence and correctness are easy to confirm while its call sites are not. Any future
consolidation must re-verify root reachability for every family that adopts the shared type,
by mutation rather than by inspection.

### Pilot D — Python wrapper reduction: the layer is not mechanical, but it hid two defects

The wrapper layer is ~27,600 lines and 921 setters across 17 files. Normalising every setter body
gives **172 distinct shapes**, and the top eight cover only 545 of the 921. That distribution is
the answer to whether the layer can be generated or collapsed: the long tail is 164 shapes over
~376 setters, each encoding something specific about its field. A generator would have to
reproduce that tail or silently change behaviour, and the two defects in "Pilot B" show that
behaviour changes in this layer are not obvious from reading it. No generator was written.

What the survey was good for was finding where the layer disagrees with itself.

**Completing the assignment-time alignment.** The earlier fix covered only the nine
`api::checked_optional` call sites. The same divergence existed in **47 more setters** using a
hand-written `T::new(x).map_err(...)` idiom — `Angle` (19), `LongitudeRequired` (7),
`LatitudeRequired` (7), `Probability` (5), `PercentageRequired` (3), `DayInterval` (2), `Gm` (2),
`Percentage` (1), `AltitudeRequired` (1). Forty-six were aligned to the contract. Each was then
verified through Python to be rejected by `validate`, KVN, and XML, because loosening a setter is
only safe when the root already catches the value.

`nominal_impact_alt` was **deliberately left eager**. `AltitudeRequired` enforces the XSD's
Earth-derived `[-430.5, 8848]`, and RDM's root does not validate it at all — that is the open
book/XSD representation conflict. Its setter is currently the only enforcement, so aligning it
would open a hole rather than close an inconsistency. It should be aligned once the representation
decision is made, not before.

**Two more root-reachability defects, found by the loosening.** Removing the eager setters exposed
which fields had no other guard. APM's `EulerAngleState`, `AngVelState`, and `InertiaState` had no
`Validate` implementation at all and were never reached from `ApmData::validate`, which visited
only the quaternion and spin blocks. An Euler angle of 400 degrees passed `validate`, KVN, and XML,
and `xmllint` rejected the emitted document on `[facet 'maxExclusive']`. All three now have
validators and are routed, with regressions covering the out-of-range, below-range, and NaN cases,
a later Euler angle, an infinite inertia moment, and the accepted `-360` boundary through the XSD
oracle.

This is the same lesson as Pilot C from the other direction: an eager wrapper setter can mask a
missing root validator indefinitely, because the surface most users touch appears to enforce the
rule.

### Pilot E — one OPM structural block: PASSED, with the model corrected twice by evidence

The representative case is OPM's **KVN keyword ordering**, which was stated three times in
`messages/opm.rs` and coupled through opaque numbers:

- `rank`, a 58-entry keyword-to-number table whose gaps (17 to 20, 26 to 30, 34 to 40, 61 to 70,
  76 to 80) silently encoded block boundaries;
- `comment_starts_block`, written in rank literals and ranges — `matches!(previous, 17 | 26 |
  30..=34 | 61 | 76)` — which are the closing ranks of each block that may precede;
- `allows_non_increasing`, written in bare ranks `70`, `76`, `80`, `25`.

Adding a keyword to the spacecraft block meant widening `30..=34` in two other functions, with
nothing but a test to catch a mistake, and nothing in the source saying the range meant "the
closing ranks of the spacecraft block".

`OPM_KVN_BLOCKS` now declares the layout once — keywords in order, how many leading and trailing
keywords are optional, whether the block repeats, whether a repeat restarts a comment run, and
whether the whole block may be absent — and all three predicates derive from it. The ranks became
an implementation detail rather than a maintained fact.

**Verification.** The refactor was done in three steps: hoist the nested functions unchanged and
confirm the suite is green; add the declaration and derived predicates alongside; prove
equivalence exhaustively over every keyword and every reachable rank before switching the parser
over. The original hand-written functions are retained in `#[cfg(test)]` scope as the golden
reference, so the equivalence proof is permanent. That is deliberately a second copy, in test
scope only: any future edit to the declaration must be shown not to change recorded behaviour.

**The model was wrong twice, and the equivalence test caught both**, which is the argument for
doing this as a checked refactor rather than a rewrite:

1. Treating the header as one block made `CLASSIFICATION` fail to open a comment run. The version
   keyword anchors the document and is its own group; the header content follows it.
2. "Repeatable" conflated two facts. Maneuvers repeat *and* a comment may open a fresh run;
   `USER_DEFINED_*` repeats *within* one logical block, so a comment there does not start a new
   one. These needed separate flags.

A third fact was missing entirely: a mandatory block between two others blocks the path, which is
why the header cannot immediately precede the metadata. Without it the derived predicate was more
permissive than the original.

**What this does not cover.** Only the KVN ordering strand. The XML sequence tables state the same
element order again, but per parent and nested, so they do not share this flat structure; the KVN
parser's match arms and the writers state it a third and fourth time. Whether those can share the
declaration is a separate experiment, and this pilot is not evidence that they can.

## Completion gate per category

A category is complete only when:

1. every normative rule has a book/XSD provenance and a P1–P4 decision;
2. every relevant root reaches the rule through populated optional and repeated containers;
3. public Rust mutation tests cover bounds, NaN, both infinities, diagnostics, and zero-byte
   streaming rejection;
4. contextual and edition/notation rules have explicit tests independent of XSD;
5. generated valid boundaries pass the reference XSD oracle, while known-invalid mutations are
   rejected before output or produce output the oracle rejects; and
6. relevant allocation and large-history benchmarks remain within their recorded budgets.

Only after two or three categories pass this gate should scalar, enum, OPM-structure, or Python
wrapper consolidation experiments begin.
