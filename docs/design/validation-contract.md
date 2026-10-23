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
mark, and KVN lines longer than 254 characters. Reject input when interpreting it would require
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

XSD value facets (bounds, patterns, enumerations, positivity) apply in every notation and to
model validation, P1 through P4, so that a valid model always converts. The one exception is a
value the book explicitly permits and the XSD forbids: the model preserves it, and XML generation
refuses it; the family record names each such field.

The books win over the XSD in one shared case: XML epochs with a timezone offset (`+hh:mm`),
bare numeric epochs, and empty epochs are rejected although `epochType` admits them, because the
books define only calendar and ordinal forms without a numeric offset (for example ODM 7.5.10). Empty mandatory
XML strings (`<OBJECT_ID/>`) are likewise rejected although `xsd:string` admits them, because the
books require mandatory values to be present (ODM 7.5.1, ADM 6.8.7). Family records link here
rather than restating this rule.

## Shared lexical decisions

These reading rules apply to every supported family and edition unless its decision record names
an exception. They are compatibility rules, not a second parser mode: reading accepts lossless
producer deviations, and writing always emits the canonical form.

- **KVN records:** accept LF, CR, CRLF, and LFCR terminators; normalizing a terminator never joins
  two records. Blank lines are allowed anywhere, and leading and trailing blanks on a record are
  insignificant, including on the version and header records. A TAB reads as a blank; writers
  never emit TAB. Content must be printable ASCII: anything else is rejected at its line and byte
  column, naming the code point. Lines longer than 254 characters are read in every family;
  writers keep the limit where the family has one. A UTF-8 byte-order mark is skipped; UTF-16 and
  UTF-32 input is rejected as "input must be UTF-8". Keywords and block markers are case-sensitive.
- **Edition first:** the `CCSDS_xxx_VERS` value (KVN) or root `version` attribute (XML) is checked
  after encoding/character checks and before the structural parse. Lexically valid input in an
  unsupported edition reports `parse.unsupported_input_version`.
- **KVN numbers:** accept ordinary signed decimal and exponent forms, including `.5`, `5.`, `1e3`,
  and mantissas longer than 16 digits. Integers accept a leading `+` (ODM 7.5.4); unsigned fields
  still refuse `-`. Preserve the nearest representable finite `f64`; reject malformed tokens,
  overflow, and nonzero values that underflow to zero. Anything after a number other than blanks
  and its unit (`25x70 [km]`, `1D-3`) is reported as "Invalid ... for KEY" at that record. A unit
  may follow its number without a blank (`1[km]`). Writers emit at most 16 significant digits,
  using the shortest spelling of the rounded value, so rewriting what was read is stable. KVN
  integers are written only within −2³¹ through 2³¹−1; wider model integers are refused. Exact
  TDM phase counts remain decimal strings.
- **XML values:** blanks around an XML number are ignored; blanks around an epoch are not. `NaN`
  and `INF` are rejected as non-finite despite `xsd:double`, except where a family record accepts
  them (OEM states and covariance). `xsi:nil` is rejected except where the CDM record documents it.
  A missing `<?xml ...?>` declaration is accepted (XML 1.0 makes it optional and UTF-8 is the
  default); a declaration that is present must open the document, and writers always emit it.
  Element nesting is limited to 16 levels.
- **Absent optional KVN values:** one shared rule covers every family and every optional value —
  number, epoch, enum, or string: a blank value or case-insensitive `N/A` means the keyword is
  absent. CDM `MANEUVERABLE` keeps `N/A` as the value the book lists for it. RDM
  `NEXT_MESSAGE_EPOCH = N/A` ("no other message is scheduled") is therefore read as absent and
  written by omission. A blank mandatory value is reported as missing at its record. Writers never
  emit an empty or `N/A` value: an optional value that would read back as absent, such as an empty
  string from XML, is refused at KVN generation, naming the keyword. XML optional strings keep
  empty values. Values are trimmed when written to KVN, as readers trim them.
- **Case:** `EULER_ROT_SEQ` reads lowercase letters and legacy ADM 1.0 numeric sequences
  in KVN/Python, writing canonical uppercase letters; XML requires its exact enum spelling.
  Keywords are uppercase. Closed enumeration values accept all-uppercase or
  all-lowercase spellings and documented aliases, but not arbitrary mixed case. Free text and
  open or ICD-defined values (frames, time systems, names) are accepted and preserved in any case,
  in every notation; the ODM 7.5.3 single-case rule is not enforced.
- **Underscore:** in a multi-word enumeration value, `_` is equivalent to a blank (CDM 6.3.2.5,
  RDM 5.3.3.4, ODM 7.5.9), so `OBJECT_TYPE = ROCKET_BODY` reads as `ROCKET BODY`. Writers emit the
  blank spelling. Free text keeps its underscores.
- **Comments:** trailing blanks on a comment line are trimmed (including fixed-width padding),
  although 7.8.5 calls them significant; leading text is kept as written.
- **Units:** check any supplied annotation against the field's allowed unit, including XML fields
  whose units are optional. Never discard an incompatible annotation or reinterpret its number.
  No automatic unit conversion is performed. Some families read alias unit spellings, such as
  uppercase `[KM**3/S**2]` in OPM and OMM; see the family records.
- **Defaults:** omitted optional fields remain absent in the model. Validators may use an explicit
  book default to interpret them; writers may emit that default when the target requires it, as a
  documented normalization. Present, empty XML elements with an explicit schema default use that
  default (TDM numeric defaults and `DATA_QUALITY=RAW`); this does not populate omitted optional
  fields. CDM's documented producer compatibility accepts empty/nil optional numeric, epoch, and
  enum XML leaves only when they carry no meaningful payload; generation omits absent values.

Each rule is implemented at the narrowest shared boundary. Family records identify exceptions,
book defaults, and edition-specific restrictions; callers do not select a stricter or looser mode.

## Preservation and normalization

Parsing and conversion preserve the supported model with the documented normalizations below;
they are not byte-preserving archival operations or a guarantee of exact model equality across
notations. Whitespace, numeric spelling, and optional fixed-unit annotations may change.

- Floating-point fields use `f64`. Readers accept longer mantissas; KVN writers emit at most 16
  significant digits, rounding values that need 17. Generation refuses rounding
  that invalidates a model constraint, such as a positive-definite covariance. XML can
  retain the full double representation. KVN rejects non-finite values and magnitudes whose
  rounded spelling would overflow. TDM phase counts follow the book's specific arbitrary-precision
  rule instead and retain exact decimal strings, subject to KVN's line-length limit.
  Floating-point negative zero is accepted on input and written as `0.0` in KVN; XML retains
  the double's sign. Opaque text and retained epoch spellings have their own family rules.
- Fixed-unit annotations may be omitted or canonicalized without changing the measurement's
  unit or applying a conversion. Optional XML annotations cannot always survive unitless KVN.
- Comment text and order are retained subject to documented whitespace normalization. Where KVN
  has fewer comment positions, a family either documents a canonical merge (OPM data/state-vector,
  OMM data/mean-elements, CDM nested data, and RDM data/atmospheric comments) or refuses
  conversion (for example, RDM comment-only optional blocks).
  Shared `UserDefined.entries` retains comment/parameter interleaving in XML. ACM also represents
  interleaved USER comments in KVN (ADM §5.3.10.5); other families retain their own block rules. Its `Comment` and
  `Parameter` variants preserve both order and exact parameter names, including names beginning
  with `USER_DEFINED_`. Outside ACM, shared user-defined KVN blocks require comments at the
  beginning; conversion refuses interleaved comments rather than relocating them.
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
`ccsds-ndm/src/messages/opm/kvn.rs`.

## Integer storage limits

Integer fields retain the native width declared by their Rust model: signed `i32` values
range from −2,147,483,648 through 2,147,483,647; `u32` values from 0 through 4,294,967,295;
`u64` values from 0 through 18,446,744,073,709,551,615. Field-specific positivity, edition,
and notation bounds still apply. These are storage limits on parsing and construction in
both Rust and Python, including XML fields whose XSD integer type has no upper bound;
out-of-range values fail rather than truncate or wrap. For example, TDM interpolation
degree uses `u32`, TDM Doppler-count scale uses `u64`, and shared CDM/RDM observation and
track counts use positive `u32`. Python's unbounded `int` does not expand these domains.
TDM phase counts are decimal strings and retain their separately documented arbitrary precision.

## OPM and OMM elapsed-time profile

OPM and OMM retain public `CalendarEpoch` fields. Epochs must have real calendar/ordinal
fields even when `TIME_SYSTEM` is MET or MRT. The full elapsed ordinal syntax of ODM
§3.2.3.2, including day `000` and days beyond the indicated year's calendar length, is
not supported by these two families. Calendar-field-valid elapsed spellings are retained
with their declared time system; no time-scale conversion or relabeling is performed.
This is a library profile restriction, not an XSD restriction. Keeping the calendar-field
contract avoids changing the shared state-vector and mean-elements APIs in this release.
OEM separately supports contextual MET/MRT elapsed fields. The regression
`conformance::values::opm_and_omm_elapsed_time_profile_is_explicit` records the OPM/OMM boundary.
