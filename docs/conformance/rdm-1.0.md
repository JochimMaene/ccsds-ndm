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
| KVN lexical and structural strictness | The shared assignment scanner rejects non-ASCII/control input, overlong lines, malformed assignments, unknown and duplicate keywords, fixed-order violations, and comments outside the beginning of one of the seven logical data blocks. RDM registers only its standard keyword order and user-defined repetition. |
| XML structure | The shared XML sequence engine is registered for the complete RDM root, header, metadata, data, and seven nested logical-block families. It rejects unknown, duplicate, and reordered children and non-schema attributes. |
| Conversion preservation | The routed-comment parser associates KVN comments with the logical block selected by the following keyword. KVN generation retains required/optional units for atmospheric, impact, state, covariance, spacecraft, and OD measures. The Annex C XML fixture survives XML-to-KVN conversion modulo the KVN COMMENT separator's non-semantic surrounding-whitespace normalization. |
| Unrepresentable XML state | An XML `<data><COMMENT>` cannot be distinguished from the first atmospheric logical-block comment in flattened KVN. All materialized and streaming KVN gates reject that state before output instead of silently moving the comment. |
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

## Disintegration compatibility alias

The bundled Annex C XML fixture (`rdm_c4.xml`) uses `MASS-LOSS + BREAKUP` for
`REENTRY_DISINTEGRATION`; the actual schema enumeration is `MASS-LOSS + BREAK-UP`.
That source fixture therefore fails independent XSD validation for that token. The library
deliberately accepts both spellings as the same `DisintegrationType::MassLossAndBreakUp`
value and emits the schema spelling in KVN and XML. It does not preserve the alias lexically.
`parsing::disintegration_alias_is_accepted_and_generates_schema_spelling` verifies
the original input, canonical output, model roundtrips, and schema-valid generated XML.

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
