# Validation contract

This document defines how the library validates mutable CCSDS NDM models. It separates semantic
validity from notation and edition representability; neither the Rust type layout nor an XSD alone
defines the complete accepted domain.

## Enforcement boundaries

- **P1 — KVN parsing:** enforce KVN syntax, the selected message edition, and self-contained
  semantic rules.
- **P2 — XML parsing:** enforce the supported XML structure, the selected message edition, XML
  value domains, and self-contained semantic rules, subject to the XML limits below.
- **P3 — `Validate::validate`:** enforce notation-neutral, self-contained model semantics,
  including rules specific to the edition recorded on the message. P3 is notation-neutral but not
  edition-neutral: `Opm`, `Oem`, and `Omm` already resolve edition-specific value domains through
  `versioning::validate_*_edition`.
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
4. Generation performs P3 before notation-specific checks. Validation and representability
   failures expose no output. A caller-provided streaming sink may fail after accepting a prefix;
   file helpers use atomic replacement to preserve the destination on failure.
5. Unknown normative values are rejected or preserved according to their governing domain; they
   are never silently rewritten to another value.
6. Runtime validation is offline. It does not fetch mutable registries or depend on network access.

## Authority and representability

CCSDS books govern normalized semantics and KVN. The bundled XSD selected for an XML edition
governs XML structure and representability. When they conflict, the library records both sources,
preserves a book-valid value in the model, and rejects conversion to an XML edition that cannot
represent it. It does not alter the value to make the conversion succeed.

Where the books are silent or unclear, or two sources conflict, parsing takes the more permissive
reading. Reading also accepts documented producer variations when the interpretation is unambiguous
and preserves the values: for example, equivalent UTF-8 XML declarations, a leading byte-order
mark, and excess trailing padding in OEM KVN. Reject input when interpreting it would require
guessing missing values, dropping content, or accepting inconsistent message semantics. Each
exception to a clear book rule is recorded in the family conformance document and tested
alongside nearby invalid cases; this policy is not a blanket relaxation of the grammar.
Generation follows the clear rules and writes a form satisfying both sources where possible
(for example, OEM XML always declares `xmlns:ndm`). A family that keeps a stricter reading lists it as a deliberate exception in its
conformance document; OEM's are under "Stricter than required" in [oem-3.0.md](../conformance/oem-3.0.md).

A conflict is resolved for the specific field and edition, with both authorities recorded in
its family document. There is no blanket rule that a schema always overrides prose or that the
narrower domain always wins. Some conflicts require a notation-specific refusal; others reflect an
explicit edition change. See the [family decision records](../conformance/README.md#family-records).

An XSD-only facet is sufficient evidence for P2 and XML P4. It is not automatically a semantic
rule for P1 or P3. For example, positivity of `gmType` and `frequencyType` is established by the
XSD but not by corresponding book text.

## Shared lexical decisions

These reading rules apply to every supported family and edition unless its decision record names
an exception. They are compatibility rules, not a second parser mode.

- **KVN lines:** accept LF, CR, CRLF, and LFCR. Normalization retains byte offsets for diagnostics.
  Keywords and block markers remain case-sensitive; content must satisfy the family's ASCII and
  line-length rules. Normalizing a terminator never joins two records.
- **KVN real numbers:** accept ordinary signed decimal and exponent forms, including `.5`, `5.`,
  `1e3`, and mantissas longer than 16 digits. Preserve the nearest representable finite `f64`;
  reject malformed tokens, overflow, and nonzero values that underflow to zero. Integers used as
  counts or identifiers retain their field-specific integer rules. Writers use the canonical
  format and precision of the selected notation. Exact TDM phase counts remain decimal strings.
- **Closed enumeration values:** KVN accepts supported all-uppercase and all-lowercase tokens,
  including documented aliases, but rejects arbitrary mixed case. XML accepts the selected XSD's
  exact spellings and specifically recorded compatibility aliases. Keywords, enum values, units,
  free text, and open registry tokens are distinct domains: unit spellings stay exact, and open
  tokens are preserved rather than uppercased or mapped to a fallback value.
- **Units:** check any supplied annotation against the field's allowed unit, including XML fields
  whose units are optional. Never discard an incompatible annotation or reinterpret its number.
  No automatic unit conversion is performed.
- **Missing values and defaults:** omitted optional fields remain absent in the model. Validators
  may use an explicit book default to interpret them; writers may emit that default when the target
  requires it, as a documented normalization. Required operational values are caller supplied.
  XML optional free-text strings retain empty values. KVN optional-value adapters accept a blank
  value or case-insensitive `N/A` as absence for optional numbers, epochs, enums, and strings;
  raw header `CLASSIFICATION` and `MESSAGE_ID` strings instead retain empty text. Field records
  identify additional notation-specific restrictions. CDM's specifically
  documented producer compatibility accepts empty/nil optional numeric, epoch, and enum leaves
  only when they carry no meaningful payload; generation omits absent values and emits standard XML.
- **Schema-only bounds:** an XSD facet is enforced on XML input/output, not automatically on KVN or
  notation-neutral validation. A separate book requirement or explicit semantic decision can
  establish a common constraint. For example, GM and sensor-frequency positivity is XML-only;
  their model/KVN values still must be finite. RDM day-interval positivity is likewise XML-only. A schema-valid double does not by itself establish
  the physical correctness of a measurement.

Each rule is implemented at the narrowest shared boundary. Family records identify exceptions,
book defaults, and edition-specific restrictions; callers do not select a stricter or looser mode.

## Preservation and normalization

Parsing and conversion preserve the supported model with the documented normalizations below;
they are not byte-preserving archival operations or a guarantee of exact model equality across
notations. Whitespace, numeric spelling, and optional fixed-unit annotations may change.

- Floating-point fields use `f64`. Ordinary KVN numbers must fit their family's 16-digit format;
  writers round values requiring 17 significant digits to at most 16. Generation refuses rounding
  that invalidates a model constraint, such as a positive-definite covariance. XML can
  retain the full double representation. KVN rejects non-finite values and magnitudes whose
  rounded spelling would overflow. TDM phase counts follow the book's specific arbitrary-precision
  rule instead and retain exact decimal strings, subject to KVN's line-length limit.
  Floating-point negative zero is accepted on input and written as `0.0` in KVN; XML retains
  the double's sign. Opaque text and retained epoch spellings have their own family rules.
- Fixed-unit annotations may be omitted or canonicalized without changing the measurement's
  unit or applying a conversion. Optional XML annotations cannot always survive unitless KVN.
- Comment text and order are retained subject to documented whitespace normalization. Where KVN
  has fewer comment positions, a family either documents a canonical merge (OPM data/state-vector
  or OMM data/mean-elements comments) or refuses conversion (for example, CDM and RDM associations).
  Shared `UserDefined.entries` retains comment/parameter interleaving in XML. Its `Comment` and
  `Parameter` variants preserve both order and exact parameter names, including names beginning
  with `USER_DEFINED_`. KVN requires comments at the beginning of the logical block, so conversion
  refuses interleaved comments rather than relocating them.
- Documented aliases normalize to the same model meaning; for example, ACM KVN `RATE_TYPE=NONE`
  means no rate columns and generates as an omitted `RATE_TYPE`. RDM accepts the compatibility
  `MASS-LOSS + BREAKUP` alias and generates `MASS-LOSS + BREAK-UP`.

Other populated information that the target notation cannot represent causes generation to fail;
it is not silently dropped. Each family's record identifies its specific normalization and refusal
rules. `validate()` checks model semantics; a successful call does not promise that every notation
can represent the model or that its physical interpretation is correct.

## Epoch storage limit

Every epoch token has a maximum retained lexical length of 64 bytes, including calendar
fractional seconds and numeric relative epochs. This is the library's fixed-storage limit,
not a bound supplied by CCSDS or the XSD. Direct constructors and KVN/XML readers reject
longer tokens rather than allocating, truncating, or rounding them. Field-specific rules,
including any fractional-second precision bound, are additional requirements. TDM's
arbitrary-precision phase-count values are separate strings and do not use epoch storage.

## XML parser limits

XML input uses the shared structural validator and typed deserializer, not a general-purpose XSD
validation engine. Known elements, attributes, namespaces, ordering, choices, and value domains
are checked. DTDs and unsupported content are rejected; processing instructions and unused
namespace declarations are ignored after limited lexical checks. In particular, malformed names
in those ignored constructs can still be accepted. This limit applies to every message family,
not only OEM. Applications requiring full XML well-formedness or independent schema validation
must validate the original document separately. Generation does not reproduce ignored constructs.

## Registry-governed values

CDM and RDM define `OBJECT_TYPE` as closed edition-specific lists. Unknown values are rejected.

OCM delegates `OBJECT_TYPE` to the living SANA Object Types registry. OCM therefore preserves the
token losslessly and validates offline syntax rather than current registry membership. Known
constants or a pinned registry snapshot may improve discoverability and conformance tests, but are
not an acceptance boundary. KVN generation emits the preserved token; XML generation rejects a
token absent from the selected bundled XSD.

## Attitude quaternion precision

ADM 504.0-B-2 annex F defines quaternion components from a rotation axis and angle, implying unit
magnitude. The shared quaternion validator used by APM, AEM and known quaternion ACM histories
checks each component in `[-1, 1]` and squared norm in `[0.999, 1.001]`. The book specifies no
numeric tolerance; this is an explicit implementation choice accommodating rounded examples,
not a new adjustable parser option. Data outside the interval is rejected rather than normalized.
The common constructor checks component domains; complete-message validation checks magnitude.

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

Per-family evidence and the common verification criteria live in [the conformance index](../conformance/README.md). Open conformance
decisions are recorded beside the rule they govern in the applicable family document and
summarized in [the support matrix](../support-matrix.md). Deliberate alias behavior (accepted
versus advertised spellings) is documented beside its `FromStr` implementation in
`ccsds-ndm/src/types.rs`; the OPM KVN ordering rationale lives beside `OPM_KVN_BLOCKS` in
`ccsds-ndm/src/messages/opm.rs`.
