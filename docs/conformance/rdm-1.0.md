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
| KVN lexical and structural strictness | The shared assignment scanner rejects non-ASCII/control input, malformed assignments, unknown and duplicate keywords, fixed-order violations, and comments outside the beginning of one of the seven logical data blocks. RDM registers only its standard keyword order and user-defined repetition. |
| XML structure | The shared XML sequence engine is registered for the complete RDM root, header, metadata, data, and seven nested logical-block families. It rejects unknown, duplicate, and reordered children and non-schema attributes. |
| Conversion preservation | The routed-comment parser associates KVN comments with the logical block selected by the following keyword. RDM 5.2.4.1 requires applicable units to be displayed, so KVN generation writes the fixed unit of every unit-bearing keyword (atmospheric, impact, metadata altitude, state, covariance, spacecraft, and OD measures) even when the model holds none, for example after reading XML that omitted an XSD-optional `units` attribute; such a value reads back from KVN with its unit set. Unitless keywords (probabilities, coefficients, `WEIGHTED_RMS`) are written without a unit. The Annex C XML fixture survives XML-to-KVN conversion modulo the KVN COMMENT separator's non-semantic surrounding-whitespace normalization. |
| XML-only comment positions | KVN has no `<data>` comment position. Following the OPM/OMM canonical merge, KVN generation writes XML `<data><COMMENT>` lines first, directly before the atmospheric block's own comments, so a KVN reparse returns them at the start of `atmosphericReentryParameters.comment` (`generation::rdm_kvn_merges_an_xml_outer_data_comment_into_the_atmospheric_block`). A comment-only `groundImpactParameters`, `spacecraftParameters`, `odParameters`, or `userDefinedParameters` block has no KVN position (its comment would attach to the next block or trail the message); all materialized and streaming KVN gates refuse it, naming the block, before writing bytes. An empty, comment-free optional XML block (`<odParameters/>`) is read as absent, as KVN would read it (`parsing::rdm_xml_empty_optional_blocks_read_as_absent_and_comment_only_blocks_are_refused_for_kvn`). |
| KVN numeric precision | Sections 5.3.3.2 and 5.3.3.3(b) limit ordinary KVN numbers to 16 decimal digits. Numeric fields use the shared number writer, rounding a value such as `0.30000000000000004` to `3.0e-1`; reparsing can change its stored `f64`. XML retains the original double. Values whose rounded spelling would overflow are refused before output. `rdm_kvn_rounds_scalars_to_sixteen_digits_and_xml_retains_precision` covers metadata, spacecraft and atmospheric fields, schema-valid XML preservation, and streaming/materialized output agreement. |
| Valid input and generation | Both shipped KVN and both shipped XML fixtures preserve their typed model in their source notation; every generated XML document validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| Shared surface contract | `library::parsing` and `library::output`, `conformance::generation`, the Python API tests, and family Criterion matrices provide common parsing, generation, diagnostics, dispatch, and workload evidence linked from the [conformance review index](README.md). |

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
| 42-52 | Mandatory atmospheric-reentry block and optional windows/confidence | `AtmosphericReentryParameters`, whose validator covers `ORBIT_LIFETIME` and its optional start/end window, with window/order and numeric tests |
| 53-84 | Ground-impact probabilities, location, and three confidence regions | `GroundImpactParameters`, whose validator covers the probabilities and every latitude/longitude bound, the nominal-impact frame/longitude/latitude co-requirement, all-or-none population and consecutive numbering per confidence interval, and strictly increasing `IMPACT_*_CONFIDENCE` |
| 85-93 | Optional state vector | `StateVector`, completeness/reference-frame tests |
| 94-117 | Optional 6x6 covariance | `OpmCovarianceMatrix`, state dependency and complete 21-element roundtrips |
| 118-129 | Optional spacecraft properties | `RdmSpacecraftParameters` (not the OPM/OMM `SpacecraftParameters`), whose validator covers the masses, areas, and coefficients, and permits signed `THRUST_ACCELERATION` while requiring it finite |
| 130-141 | Optional orbit-determination information | `OdParameters`, shared with CDM and routed from `RdmData::validate`; its validator covers the counts, spans, and percentages |
| 142 | User-defined parameters | typed user-defined map, strict scanner and roundtrips |

No annex-A RDM feature is absent. `OBJECT_TYPE` is the exception to caller-supplied vocabulary:
RDM defines it as a closed edition-specific list, so unknown values are rejected rather than
preserved — see "Registry-governed values" in the [validation contract](../design/validation-contract.md).
Other SANA-sourced vocabulary remains external and caller supplied, while locally decidable
controlled values and numeric ranges are typed and validated.

RDM §§3.5.8–3.5.9 say the orbit-lifetime and re-entry epochs *should* resolve consistently. That
is advisory rather than a `shall`, and reconciling them needs time-system context the block does
not supply, so the two are not compared. Recorded here as a deliberate non-claim.

`NOMINAL_IMPACT_ALT` separates semantics from representability. RDM states no numeric range and
permits non-Earth body-fixed frames, while the common 4.0 XSD applies an Earth-derived range, so
the model preserves any finite altitude and validation enforces only finiteness. KVN writes such a
value; XML generation refuses it with the offending field path rather than altering it to fit.
Both XSD boundaries generate schema-valid XML.

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

Other accepted deviations, each read losslessly and written in canonical form:

- a missing optional unit in KVN or XML (KVN output displays the fixed unit, see above);
- mixed-case free text (`OBJECT_NAME`, frames, registry-sourced names), preserved as written;
- `_` for a blank in multi-word enumeration values (`ROCKET_BODY`), per RDM 5.3.3.4;
- `INTERNATIONAL_DESIGNATOR` values that do not follow the `YYYY-NNNP{PP}` format: the format is
  a recommendation for registry-derived values, and `UNKNOWN` is permitted, so it is not enforced;
- an `IMPACT_REF_FRAME` naming any body-fixed frame; frames are SANA-registry values and are not
  checked against the center.

`NEXT_MESSAGE_EPOCH = N/A` ("no other message is scheduled", RDM 3.4) is read as absent like every
blank or `N/A` optional KVN value, and absence is written by omitting the keyword; the XSD epoch
type has no `N/A` spelling. See "Absent optional KVN values" in the
[validation contract](../design/validation-contract.md#shared-lexical-decisions).

XML epochs follow the shared book-over-XSD epoch rule stated once in the
[validation contract](../design/validation-contract.md#authority-and-representability): offset,
bare numeric, and empty `epochType` values are rejected.

Unit spellings follow the book tables (`m**2`, `kg/m**2`, `m/s**2`, `km**2/s`).

## Allocation and packaged-surface evidence

`fixed_family_allocations` fixes strict KVN parse budgets at 96 allocations/12,000 bytes and
preflighted streaming generation at 40 allocations/2,000 bytes. `test_rdm.py` and the shared Python
options matrix cover both notations, files, epochs, generic identity, diagnostics, and limits.
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

All four book line endings (LF, CR, CRLF and LFCR) are accepted with identical models and
byte-position-preserving normalization.

## Atmospheric lifetime values

The book describes `ORBIT_LIFETIME` and its window offsets as double-precision times until
re-entry and gives no zero or negative value a meaning. Following the shared rule that XSD facets
apply in every notation (see the
[validation contract](../design/validation-contract.md#authority-and-representability)), the
`dayIntervalTypeUR` positivity is enforced in KVN, XML, and model validation for all three
lifetime fields, so a valid model always converts. Errors name the keyword. Existing
lifetime-window start/end ordering checks still apply.
`generation::orbit_lifetime_rule_is_enforced_at_every_public_boundary` and
`generation::lifetime_window_offsets_are_positive_in_every_notation` cover the rule.

Range errors from KVN or XML input name the keyword (`PROBABILITY_OF_IMPACT`, `TRACKS_USED`,
`NOMINAL_IMPACT_LON`, `ORBIT_LIFETIME_CONFIDENCE_LEVEL`, ...) rather than a Rust type; they are
reported by model validation with the field path (`parsing::rdm_range_errors_name_the_keyword_in_both_notations`).

## Confidence-only locations and OD count profile

The nominal-location dependency in RDM §3.5.10 is applied when any nominal impact
longitude, latitude, or altitude is supplied, including `NOMINAL_IMPACT_ALT` alone: frame,
nominal longitude, and nominal latitude are then required together. A complete
confidence-region block (`IMPACT_n_*`) alone does not trigger that dependency, so confidence
coordinates without `IMPACT_REF_FRAME` are accepted. This explicitly follows the schema's independently optional fields rather than
requiring an additional nominal solution. The broader prose phrase "ground impact location"
could also be read to include a confidence-only region; this library takes the permissive
reading. If its frame is omitted, the caller must establish the coordinate interpretation
with the exchange partner; the library supplies no implicit frame.
`rdm::generation::confidence_only_region_follows_the_schema_without_inventing_a_nominal_solution`
records this boundary.

Longitude and latitude ranges are cited as RDM §3.5.11 and §3.5.12, the numbering in the
corrigendum-1 text used here (3.5.14 and 3.5.15 there cover which `IMPACT_n_*` sets to use).

The shared CDM/RDM observation and track counts require positive `u32` in both notations,
although the book describes integer counts without a separate positivity requirement.
This deliberate interoperable profile follows the XSD rather than applying its positivity
only to XML. Zero is rejected, not converted to absence. See the
[CDM count decision](cdm-1.0.md#categorical-aliases-and-od-count-profile) and
[integer storage limits](../design/validation-contract.md#integer-storage-limits).
