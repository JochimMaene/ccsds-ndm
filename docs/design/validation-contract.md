# Validation contract

This document defines how the library validates mutable CCSDS NDM models. It separates semantic
validity from notation and edition representability; neither the Rust type layout nor an XSD alone
defines the complete accepted domain.

## Enforcement boundaries

- **P1 — KVN parsing:** enforce KVN syntax, the selected message edition, and self-contained
  semantic rules.
- **P2 — XML parsing:** enforce XML syntax and structure, the selected message edition, XML value
  domains, and self-contained semantic rules.
- **P3 — `Validate::validate`:** enforce notation-neutral, self-contained model semantics,
  including rules specific to the edition recorded on the message. P3 is notation-neutral but not
  edition-neutral: `Opm`, `Oem`, and `Omm` already resolve edition-specific value domains through
  `versioning::validate_*_edition`. See [Edition-aware validation](edition-aware-validation.md).
- **P4 — generation:** enforce P3 plus the selected notation and edition's representability rules.

Models remain conveniently mutable. They may be temporarily invalid between edits, but every
public generation path validates the complete graph before writing any caller-visible bytes.

## Rules

1. Each reusable invariant has one authoritative implementation at the narrowest shared level.
   Leaf constraints belong to the leaf type; field-, block-, notation-, and edition-specific rules
   remain contextual.
2. KVN and XML accept the same semantic value domain for a field and edition where their governing
   standards agree. Legal lexical spellings may differ when the standard says they differ.
3. Root validation revisits every relevant child, including populated optional blocks and every
   element of repeated collections. A correct but unreachable child validator is a defect.
4. Generation performs P3 before notation-specific checks. Rejected materialized, streaming, and
   file output must not expose a partial document.
5. Unknown normative values are rejected or preserved according to their governing domain; they
   are never silently rewritten to another value.
6. Runtime validation is offline. It does not fetch mutable registries or depend on network access.

## Authority and representability

CCSDS books govern normalized semantics and KVN. The bundled XSD selected for an XML edition
governs XML structure and representability. When they conflict, the library records both sources,
preserves a book-valid value in the model, and rejects conversion to an XML edition that cannot
represent it. It does not alter the value to make the conversion succeed.

Known conflicts include:

- OCM `DC_PA_START_ANGLE` and `DC_PA_STOP_ANGLE`: ODM permits any finite magnitude; OCM 3.0 XML
  uses `angleType`, whose range is `[-360, 360)`. **Resolved** with the P3/P4 split: finiteness at
  P3, refused by `Ocm::validate_xml_representability` at XML generation.
- OPM `MAN_DELTA_MASS`: the authorities do differ, and the resolution is per edition rather than a
  general rule.

  | Authority | Says | Applies to |
  | --- | --- | --- |
  | 502.0-B-3 §3.2.4.7 | "MAN_DELTA_MASS may be used for both finite and impulsive maneuvers; the value must be a negative number" | prose, both editions |
  | `ndmxml-2.0.0-common-2.0.xsd` `deltamassType` → `negativeDouble` (`maxExclusive 0.0`) | strictly negative | OPM 2.0 XML |
  | `ndmxml-4.0.0-common-4.0.xsd` `deltamassTypeZ` → `nonPositiveDouble` (`maxInclusive 0.0`) | zero permitted | OPM 3.0 XML |

  The 3.0 schema states its own reason inline: "Type Z for deltamass that allows value of zero
  (attitude maneuvers)". That annotated widening is the later and more specific statement for the
  3.0 edition, so zero is accepted there and rejected in 2.0, which is what
  `versioning::validate_opm_edition` implements. Positive values are rejected in both.

  This resolution is scoped to this field and these two editions. It is **not** a rule that a
  schema beats prose, any more than the earlier draft's "the narrower semantic rule applies" was a
  rule that prose beats a schema. Each conflict is resolved on its own evidence and recorded here
  with citations, so a reader can check the reasoning rather than apply a slogan.

- OCM `DAYS_SINCE_FIRST_OBS` and `DAYS_SINCE_LAST_OBS`: ODM permits signed values; the OCM 3.0 XSD
  uses a non-negative day interval. **Resolved** with the P3/P4 split: finiteness at P3, negative
  values refused at XML generation.
- RDM `NOMINAL_IMPACT_ALT`: RDM provides no numeric range and permits non-Earth body-fixed frames;
  the common 4.0 XSD applies an Earth-derived altitude range. **Resolved** as the worked example of
  the policy above: the model preserves any finite altitude, P3 enforces finiteness only, KVN
  writes it, and XML generation refuses a value outside `[-430.5, 8848]` rather than altering it.
- TDM `ANGLE_1` and `ANGLE_2`: TDM 2.0 §§3.5.4.2–3.5.4.3 require `[-180, 360)`; the shared
  `angleRange` type is the wider `[-360, 360)`. The narrower semantic rule applies.
- TDM `TEMPERATURE`, `RCS`, and `STEC`: TDM 2.0 §§3.5.8.3, 3.5.5.2, and 3.5.7.1 require positive
  values; the TDM 2.0 XSD uses `nonNegativeDouble` for `TEMPERATURE` and unconstrained `xsd:double`
  for `RCS` and `STEC`. The narrower semantic rule applies.

An XSD-only facet is sufficient evidence for P2 and XML P4. It is not automatically a semantic
rule for P1 or P3. For example, positivity of `gmType` and `frequencyType` is established by the
XSD but not by corresponding book text.

## Registry-governed values

CDM and RDM define `OBJECT_TYPE` as closed edition-specific lists. Unknown values are rejected.

OCM delegates `OBJECT_TYPE` to the living SANA Object Types registry. OCM therefore preserves the
token losslessly and validates offline syntax rather than current registry membership. Known
constants or a pinned registry snapshot may improve discoverability and conformance tests, but are
not an acceptance boundary. KVN generation emits the preserved token; XML generation rejects a
token absent from the selected bundled XSD.

## Evidence required for a constraint

The maintained evidence records:

| Column | Meaning |
| --- | --- |
| Requirement | Stable local identifier for the rule |
| Book source | Publication, edition, and section/table |
| XSD source | Schema type, facets, and edition |
| Enforcement | Required P1–P4 boundaries |
| Reachability | Root-to-field route, including optional and repeated containers |
| Tests | Boundary, mutation, conversion, and diagnostic evidence |

Coverage requires more than valid fixture round trips. Tests must construct optional blocks that
fixtures omit and mutate repeated elements beyond index zero. Numeric tests cover NaN, both
infinities, boundaries, and adjacent invalid values where meaningful. Tests also assert useful
field paths and zero-byte streaming failures.

## XSD oracle policy

XSD validators are tested before their negative verdicts are trusted. Xerces is the reference for
numeric-facet negative tests because libxml2 accepts NaN against bounding facets. `xmllint` remains
useful for fast positive schema checks. The suite contains a valid control and a known-invalid NaN
document; direct Rust tests remain authoritative for prose-only requirements such as finiteness.

## Implementation guidance

Prefer compositional validation: leaf predicates reused by block validators, and block validators
reached by small root-routing methods. A serialization walk may detect non-finite numbers but
cannot infer positive durations, probabilities, cross-field requirements, edition rules, or good
field paths, so it is not the default design.

Static field-name searches are useful for triage only. Same-name fields, parent-validates-child
patterns, helper functions, and notation-specific preflights make them unsound as coverage proof.

The current evidence assessment and ordered implementation categories are maintained in
[Validation evidence and rollout](validation-rollout.md).
