# RDM 1.0 conformance

This inventory records maintainer evidence for standalone RDM 1.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: RDM 1.0.
- Semantic and KVN authority: CCSDS 508.1-B-1 with corrigendum 1
  (`data/rdm_508x1b1c1.pdf`), principally sections 3 and 5 and annex C.
- XML authority: NDM/XML schema set 4.0.0 and RDM schema 1.0.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and notation conversion through the shared Rust generation gate.

## Executable evidence

The `rdm` suite establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and structural strictness | The shared assignment scanner rejects non-ASCII values outside COMMENT payloads and forbidden control input, malformed assignments, unknown and duplicate keywords, fixed-order violations, and comments outside the beginning of one of the seven logical data blocks. RDM registers only its standard keyword order and user-defined repetition. |
| XML structure | The shared XML sequence engine is registered for the complete RDM root, header, metadata, data, and seven nested logical-block families. It rejects unknown, duplicate, and reordered children and non-schema attributes. |
| Conversion preservation | The routed-comment parser associates KVN comments with the logical block selected by the following keyword. RDM 5.2.4.1 requires applicable units to be displayed, so KVN generation writes the fixed unit of every unit-bearing keyword (atmospheric, impact, metadata altitude, state, covariance, spacecraft, and OD measures) even when the model holds none, for example after reading XML that omitted an XSD-optional `units` attribute; such a value reads back from KVN with its unit set. Unitless keywords (probabilities, coefficients, `WEIGHTED_RMS`) are written without a unit. The Annex C XML fixture survives XML-to-KVN conversion modulo the KVN COMMENT separator's non-semantic surrounding-whitespace normalization. |
| XML-only comment positions | KVN has no `<data>` comment position. Following the OPM/OMM canonical merge, KVN generation writes XML `<data><COMMENT>` lines first, directly before the atmospheric block's own comments, so a KVN reparse returns them at the start of `atmosphericReentryParameters.comment` (`generation::rdm_kvn_merges_an_xml_outer_data_comment_into_the_atmospheric_block`). A comment-only `groundImpactParameters`, `spacecraftParameters`, `odParameters`, or `userDefinedParameters` block has no KVN position (its comment would attach to the next block or trail the message); all materialized and streaming KVN gates refuse it, naming the block, before writing bytes. An empty, comment-free optional XML block (`<odParameters/>`) is read as absent, as KVN would read it (`parsing::rdm_xml_empty_optional_blocks_read_as_absent_and_comment_only_blocks_are_refused_for_kvn`). |
| KVN numeric precision | Sections 5.3.3.2 and 5.3.3.3(b) limit ordinary KVN numbers to 16 decimal digits. Numeric fields use the shared number writer, rounding a value such as `0.30000000000000004` to `0.3`; reparsing can change its stored `f64`. XML retains the original double. Values whose rounded spelling would overflow are refused before output. `rdm_kvn_rounds_scalars_to_sixteen_digits_and_xml_retains_precision` covers metadata, spacecraft and atmospheric fields, schema-valid XML preservation, and streaming/materialized output agreement. |
| Valid input and generation | Both shipped KVN and both shipped XML fixtures preserve their typed model in their source notation; every generated XML document validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| Shared surface contract | `library::parsing` and `library::output`, `conformance::generation`, the Python API tests, and family Criterion matrices provide common parsing, generation, diagnostics, dispatch, and workload evidence linked from the [conformance review index](README.md). |

KVN generation also checks both adjacent `IMPACT_*_CONFIDENCE` pairs after numeric rounding.
If two distinct levels would become equal, generation refuses before writing bytes rather than
violating the strict ordering required by §3.5.16. XML retains their original precision.
`generation::kvn_refuses_confidence_levels_that_round_to_the_same_value` covers both pairs,
schema-valid XML preservation, and string/streaming KVN refusal.

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
| 42-52 | Mandatory atmospheric-reentry block and optional windows/confidence | `AtmosphericReentryParameters`, whose validator checks positive numeric lifetime values; `RdmData::validate` checks lifetime-window ordering; absolute epoch-window limits are stated below |
| 53-84 | Ground-impact probabilities, location, and three confidence regions | `GroundImpactParameters`, whose validator covers the probabilities and every latitude/longitude bound, the nominal-impact frame/longitude/latitude co-requirement, all-or-none population and consecutive numbering per confidence interval, and strictly increasing `IMPACT_*_CONFIDENCE` |
| 85-93 | Optional state vector | `StateVector`, completeness/reference-frame tests |
| 94-117 | Optional 6x6 covariance | `OpmCovarianceMatrix`, state dependency and complete 21-element roundtrips |
| 118-129 | Optional spacecraft properties | `RdmSpacecraftParameters` (not the OPM/OMM `SpacecraftParameters`), whose validator covers the masses, areas, and coefficients, and permits signed `THRUST_ACCELERATION` while requiring it finite |
| 130-141 | Optional orbit-determination information | `OdParameters`, shared with CDM and routed from `RdmData::validate`; its validator covers the counts, spans, and percentages |
| 142 | User-defined parameters | ordered user-defined entries, scanner and roundtrips |

No annex-A RDM feature is absent. `OBJECT_TYPE` is not registry-governed: RDM defines it as a
closed edition-specific list, so unknown values are rejected rather than preserved — see
"Registry-governed values" in the [validation contract](../design/validation-contract.md).
SANA-sourced vocabulary remains external and caller supplied, while locally decidable
controlled values and numeric ranges are typed and validated.

RDM §§3.5.8–3.5.9 say the orbit-lifetime and re-entry epochs *should* resolve consistently. That
is advisory rather than a `shall`, and reconciling them needs time-system context the block does
not supply, so the two are not compared. Recorded here as a deliberate non-claim.

`NOMINAL_IMPACT_ALT` separates semantics from representability. RDM states no numeric range and
permits non-Earth body-fixed frames, while the common 4.0 XSD applies an Earth-derived range, so
the model preserves any finite altitude and validation enforces only finiteness. KVN writes such a
value; XML input rejects it and XML generation refuses it with the offending field path rather
than altering it to fit, because the XSD governs XML value domains. Both XSD boundaries generate
schema-valid XML. `altType` makes the metre `units` annotation
optional: XML input retains omission, XML output may omit it, and KVN output supplies `[m]`
under §5.2.4.1. The Rust measure is `ImpactAltitude`, whose `units` field is
`Option<LengthUnits>`. Python exposes the numeric altitude.
`parsing::nominal_impact_altitude_units_are_optional_in_xml_but_displayed_in_kvn` covers the
schema-valid omission, XML preservation, and canonical KVN output.

All public validation and generation routes revisit mutable atmospheric values:
`ORBIT_LIFETIME_CONFIDENCE_LEVEL` remains within `[0, 100]`, including after direct field
mutation, and `REENTRY_ALTITUDE` and optional `DRAG_PARAMETERS_ALTITUDE` must be finite.
These altitude fields retain their sign; no extra physical altitude range is inferred.

## Tolerated deviations and value spellings

`REENTRY_DISINTEGRATION = MASS-LOSS + BREAKUP` is a tolerated transcription variant that is in
neither the book nor the XSD: the book spells `BREAK-UP` throughout, and its Figure C-4 breaks
`BREAK-` / `UP` across a line, which is how a copy loses the hyphen. Both notations read it as
`DisintegrationType::MassLossAndBreakUp` and always write `MASS-LOSS + BREAK-UP`; it is not
preserved lexically, and diagnostics list only the book spellings. The bundled `rdm_c4.xml`
fixture uses the book spelling; `parsing::disintegration_alias_is_accepted_and_generates_schema_spelling`
covers the variant.

`REENTRY_UNCERTAINTY_METHOD` and `IMPACT_UNCERTAINTY_METHOD` accept exactly the XSD values `NONE`,
`ANALYTICAL`, `STOCHASTIC`, and `EMPIRICAL`. Like the other closed RDM enumerations, these and
`REENTRY_DISINTEGRATION` also accept the all-lowercase spelling in KVN (see "Case" in the
[validation contract](../design/validation-contract.md#shared-lexical-decisions)); XML follows the
XSD enumeration.

Other accepted spellings and compatibility exceptions. Normalization is field-specific: closed
enumerations are written in canonical form, while free text, `INTERNATIONAL_DESIGNATOR`, and
`IMPACT_REF_FRAME` are written as read:

- a missing unit in KVN, which 5.2.4.1 says to display (KVN output displays the fixed unit, see
  above; omission of XSD-optional XML `units` is schema-valid, not a deviation);
- mixed-case free text (`OBJECT_NAME`, frames, registry-sourced names), read and also written as
  given, although KVN §5.2.3.3 requires uppercase text values and XML §5.4.3.5 forbids mixing
  cases: under the shared policy the library does not alter the case of free text, since
  case-folding a caller's identifier would change data the library cannot reinterpret;
- `_` for a blank in multi-word enumeration values (`ROCKET_BODY`) in KVN and Python
  categorical lookup, per RDM 5.3.3.4; XML uses its schema spellings, except the explicitly
  documented disintegration transcription alias above;
- `INTERNATIONAL_DESIGNATOR` values that do not follow the `YYYY-NNNP{PP}` format: table 3-2
  mandates that layout and provides `UNKNOWN` for objects without a designator. The library
  deliberately follows the XSD's `xsd:string` domain instead, accepting any nonempty value
  without rewriting producer identifiers. This is a compatibility departure from mandatory
  book prose, not a claim that the book's layout is optional. The regression
  `generation::producer_designators_follow_the_xsd_string_domain` checks both notation
  round trips and schema-valid XML;
- an `IMPACT_REF_FRAME` naming any body-fixed frame; frames are SANA-registry values and are not
  checked against the center.

`NEXT_MESSAGE_EPOCH = N/A` ("no other message is scheduled", RDM 3.4) is read as absent like every
blank or `N/A` optional KVN value, and absence is written by omitting the keyword; the XSD epoch
type has no `N/A` spelling. See "Absent optional KVN values" in the
[validation contract](../design/validation-contract.md#shared-lexical-decisions).

XML epochs follow the shared book-over-XSD epoch rule stated once in the
[validation contract](../design/validation-contract.md#authority-and-representability): nonzero offset,
bare numeric, and empty `epochType` values are rejected.

Unit spellings follow the book tables (`m**2`, `kg/m**2`, `m/s**2`, `km**2/s`).

## Allocation and packaged-surface evidence

`fixed_family_allocations` fixes strict KVN parse budgets at 96 allocations/12,000 bytes and
preflighted streaming generation at 40 allocations/2,000 bytes. `test_rdm.py` and the shared Python
workflow matrix cover both notations, files, epochs, generic identity, diagnostics, and the fixed XML nesting limit.
Strict binding audit, generated stubs/doc checks, wheel verification, and Rust artifact
verification are the packaged gates.

## Verification outcome

RDM 1.0 meets the common [verification criteria](README.md#verification-criteria) for
self-contained format and model validation on Rust and Python. The requirement inventory,
family tests, shared public-route tests, and packaged tests are complementary evidence;
none alone establishes conformance or exhaustive mutation coverage.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

## User-defined order and line endings

User-defined XML blocks retain comments and parameters in one ordered `entries` collection,
including comments between parameters. XML generation preserves that order. Flattened RDM KVN
can retain comments only before the first user-defined parameter; conversion rejects an
interleaved, empty or comments-only user-defined block before writing bytes. Ordinary leading
comments and all parameter values retain their order through KVN parsing and generation.
Repeated parameter names are preserved in their original order in both notations. Names must
have a nonempty suffix after `USER_DEFINED_`. XML parameter names that contain lowercase
letters or blanks cannot be represented by RDM's KVN name grammar and are refused on KVN
output rather than rewritten.

All four book line endings (LF, CR, CRLF and LFCR) are accepted with identical models and
byte-position-preserving normalization.

## Atmospheric lifetime values

The book describes `ORBIT_LIFETIME` and its window offsets as double-precision times until
re-entry and gives no zero or negative value a meaning. Following the shared rule that XSD facets
apply in every notation (see the
[validation contract](../design/validation-contract.md#authority-and-representability)), the
`dayIntervalTypeUR` positivity is enforced in KVN, XML, and model validation for all three
lifetime fields, to keep that value domain compatible across notations. Model-validation errors
name the keyword. Typed XML measures can fail deserialization first with a common type name
and structured source location instead of a model path; see the shared diagnostics contract.
Numeric lifetime-window start/end ordering is checked in `RdmData::validate`: START must be no
later than END. Table 3-3 defines them as the start and end of one window, so a reversed pair is
inconsistent message semantics, which the
[validation contract](../design/validation-contract.md#authority-and-representability) rejects.
The XSD has no cross-field constraint; this is not a claim that it enforces the relationship.
`generation::orbit_lifetime_rule_is_enforced_at_every_public_boundary` and
`generation::lifetime_window_offsets_are_positive_in_every_notation` cover the rule.

Absolute reentry and impact epoch windows are checked for valid epoch syntax and precision,
but their start/end ordering and enclosure of a nominal epoch are not checked. These are
separate from the numeric lifetime-window checks above. The XSD represents their epochs as
independently optional values; the caller remains responsible for establishing the window's
physical consistency. RDM §§3.5.6–3.5.9 also contain recommendations, which are not additional
parser rejection rules.

For unwrapped scalar fields (`PROBABILITY_OF_IMPACT`, `TRACKS_USED`, `NOMINAL_IMPACT_LON`,
`ORBIT_LIFETIME_CONFIDENCE_LEVEL`, ...), model validation names the keyword and field path
(`parsing::rdm_range_errors_name_the_keyword_in_both_notations`). Typed XML measures such as
WET_MASS and ORBIT_LIFETIME may instead fail deserialization with schema/type wording;
these retain `parse.xml.syntax` and a structured location. A canonical model path is not
always available at that boundary.

## Confidence-only locations and OD count profile

The nominal-location dependency in RDM §3.5.10 is applied when any nominal impact
longitude, latitude, or altitude is supplied, including `NOMINAL_IMPACT_ALT` alone: frame,
nominal longitude, and nominal latitude are then required together. For confidence regions
(`IMPACT_n_*`) without a nominal location the book is ambiguous: table 3-3 makes
IMPACT_REF_FRAME mandatory only "if NOMINAL_IMPACT_LON and NOMINAL_IMPACT_LAT are present", and
§3.5.10's "if a ground impact location is given" does not say whether a region alone is one, yet
table 3-3 gives the `IMPACT_n_*` longitudes and latitudes "with respect to the value of
IMPACT_REF_FRAME". Reading therefore accepts a frameless region (no frame or nominal value is
invented), while KVN and XML generation refuse one, naming IMPACT_REF_FRAME, so the library
never writes coordinates without their frame.
`rdm::generation::confidence_only_region_follows_the_schema_without_inventing_a_nominal_solution`
checks frameless reading in both notations, output refusal, and preservation with a frame.

Longitude and latitude ranges are cited as RDM §3.5.11 and §3.5.12, the numbering in the
corrigendum-1 text used here (3.5.14 and 3.5.15 there cover which `IMPACT_n_*` sets to use).

The shared CDM/RDM observation and track counts require positive `u32` in both notations,
although the book describes integer counts without a separate positivity requirement.
This deliberate interoperable profile follows the XSD rather than applying its positivity
only to XML. Zero is rejected, not converted to absence. See the
[CDM count decision](cdm-1.0.md#categorical-aliases-and-od-count-profile) and
[integer storage limits](../design/validation-contract.md#integer-storage-limits).

KVN reading tolerates lines longer than the book's 254-character limit where applicable;
writing enforces that limit, so an accepted overlong record can require shortening before
regeneration. Shared encoding, namespace, epoch, `N/A`, whitespace and categorical rules
are specified in the [validation contract](../design/validation-contract.md).

KVN reading permits omitted unit annotations, including those 5.2.4.1 says to display;
the field's fixed unit is implied. XML still requires a units attribute wherever the XSD
requires one. Omission on KVN input never performs unit conversion. Python model setters
may defer scalar-domain errors to validate()/generation; constructor/setter timing is not
a uniform eager-validation guarantee. To omit NEXT_MESSAGE_EPOCH in Python, use None;
the string N/A is an input-document sentinel. Flattened KVN data comments merge into the
first represented logical block, as in the shared ODM convention.

The library does not infer physical consistency beyond its stated scalar and conditional checks.
In particular, finite signed cross-track distances are retained; DRY_MASS is not compared to
WET_MASS, observation/track counts are not compared to their available counts, and absolute
impact/reentry windows are not ordered or checked for enclosure. These relationships remain
the caller's responsibility. This does not relax the positive count and nonnegative mass domains, nominal
location dependencies, confidence-region completeness, or increasing confidence levels.

XML input and output both enforce the bundled NOMINAL_IMPACT_ALT range −430.5…8848 m.
The model and KVN preserve any finite altitude under the documented book/schema conflict.
