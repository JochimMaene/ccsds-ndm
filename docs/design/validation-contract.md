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
- **P4 — generation:** enforce P3 plus the selected notation and edition's representability rules,
  except for the [OCM XML linkage exception](#ocm-xml-linkage-exception) below.

Models remain conveniently mutable. They may be temporarily invalid between edits, but every
public generation path validates the complete graph of represented content before writing any
caller-visible bytes. OCM XML uses the [linkage exception](#ocm-xml-linkage-exception) below.

## Rules

1. Each reusable invariant has one authoritative implementation at the narrowest shared level.
   Leaf constraints belong to the leaf type; field-, block-, notation-, and edition-specific rules
   remain contextual.
2. KVN and XML accept the same semantic value domain for a field and edition where their governing
   standards agree. Legal lexical spellings may differ when the standard says they differ.
3. Root validation revisits every relevant child, including populated optional blocks and every
   element of repeated collections. A correct but unreachable child validator is a defect.
4. Generation performs P3 before notation-specific checks, subject to the OCM XML linkage
   exception above. Validation and representability
   failures expose no output. A caller-provided streaming sink may fail after accepting a prefix;
   file helpers use atomic replacement to preserve the destination on failure.
5. Unknown normative values are rejected or preserved according to their governing domain; they
   are never silently rewritten to another value.
6. Runtime validation is offline. It does not fetch mutable registries or depend on network access.

## Authority and representability

CCSDS books govern normalized semantics and KVN. The bundled XSD selected for an XML edition
governs XML structure, representability, and schema-defined value domains. Explicit book
requirements, including conditional dependencies, also apply when the schema cannot express them.
An optional XSD element does not cancel a book requirement conditional on another field.
Defaults and recommendations do not create additional mandatory fields. When authorities conflict, the library records both sources,
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
(for example, every generated XML root declares `xmlns:ndm`). A family that keeps a stricter reading lists it as a deliberate exception in its
conformance document; OEM's are under "Stricter than required" in [oem-3.0.md](../conformance/oem-3.0.md).

A conflict is resolved for the specific field and edition, with both authorities recorded in
its family document. There is no blanket rule that a schema always overrides prose or that the
narrower domain always wins. Some conflicts require a notation-specific refusal; others reflect an
explicit edition change. See the [family decision records](../conformance/README.md#family-records).

XSD value facets (bounds, patterns, enumerations, positivity) apply in every notation and to
model validation, P1 through P4, to keep those value domains compatible across notations.
This does not guarantee conversion: target structure, lexical limits, and preservation checks
still apply. When the book explicitly permits a value the XSD forbids, the model preserves it
and XML generation refuses it; the family record names each such field.

Book-over-XSD epoch constraints are explicit: absolute epoch fields use four-digit,
nonnegative years and calendar or ordinal dates, without nonzero timezone offsets (`+hh:mm`),
empty fractional seconds (`15.`), or empty values. ODM 7.5.10, ADM 6.8.9 and their
CDM/RDM counterparts limit fractional seconds to 16 digits; this cap remains enforced
rather than broadening the shared validation and generation contract in this release.
TDM has no fractional-digit cap. Negative OPM/OMM/OEM MET/MRT elapsed epochs remain unsupported; OCM/ACM SI-second
relative offsets may be negative. The lossless input aliases `+00:00` and `-00:00` normalize
to `Z` without changing `TIME_SYSTEM` or performing a time-scale conversion. Nonzero offsets
remain unsupported. The 64-byte limit is checked on the supplied token before normalization.
OCM and ACM relative fields and histories accept numeric SI-second tags, including
exponents expanded exactly to plain decimal; their absolute reference anchors remain
calendar/ordinal. These limits are narrower than the XSD epoch union. The 64-byte
lexical storage limit applies independently and never truncates a token. Empty mandatory
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
  never emit TAB. Keywords and ordinary values must be printable ASCII. COMMENT payloads also
  accept printable UTF-8 as a producer extension and preserve their text. XML can retain those
  comments; KVN generation refuses non-ASCII comments before output instead of transliterating
  or deleting characters. Other forbidden characters are rejected at their line and byte column,
  naming the code point. An unterminated final record is accepted in every family. The version
  is the first nonblank record; comments before it are rejected. Lines longer than 254 characters are read in every family;
  writers keep the limit where the family has one. A UTF-8 byte-order mark is skipped; UTF-16 and
  UTF-32 input is rejected as "input must be UTF-8". Keywords and block markers are case-sensitive.
- **Edition first:** the `CCSDS_xxx_VERS` value (KVN) or root `version` attribute (XML) is checked
  after encoding/character checks and before the structural parse. A recognized version record with an
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
  OCM maneuver columns retain numeric text rather than parsing it into model doubles; nonzero
  underflow text remains available in the model/XML, and KVN generation refuses it as
  unrepresentable. Shared three-component vectors and KVN four-component vectors use the
  checked double reader and reject nonzero underflow, retaining representable subnormals.
- **XML values:** literal TAB/LF/CR in attribute values normalize to spaces, with CRLF counting
  as one space (XML 1.0 §§2.11/3.3.3). Character references retain their referenced characters;
  writers use references to preserve TAB/LF/CR in attributes. Element-text TAB/LF remain text.
  Blanks around an XML number are ignored; blanks around an epoch are not. `NaN`
  and `INF` are rejected as non-finite despite `xsd:double`, except where a family record accepts
  them (OEM states/covariance and OCM time offsets). `xsi:nil` is rejected; CDM separately accepts the unqualified
  producer attribute `nil` on empty optional leaves, as documented in its family record.
  A missing `<?xml ...?>` declaration is accepted (XML 1.0 makes it optional and UTF-8 is the
  default); a declaration that is present must open the document, and writers always emit it.
  Element nesting is limited to 16 levels.
- **Absent optional KVN values:** one shared rule covers every family and every optional value —
  number, epoch, enum, or string: a blank value or case-insensitive `N/A` means the keyword is
  absent. CDM `MANEUVERABLE` keeps `N/A` as the value the book lists for it. RDM
  `NEXT_MESSAGE_EPOCH = N/A` ("no other message is scheduled") is therefore read as absent and
  written by omission. A blank or `N/A` mandatory value is reported as missing at its record. Writers never
  emit an empty or `N/A` standard-field value: an optional value that would read back as absent, such as an empty
  string from XML, is refused at KVN generation, naming the keyword. XML optional strings keep
  empty values. In CDM XML, case-insensitive `N/A` leaves are absent too (CDM 1.4.3.1), except
  `MANEUVERABLE`; attributes and fixed units are checked before omission. Other families' XML
  text fields retain literal `N/A`; numeric/epoch/enumeration fields must satisfy their schema type.
  CDM XML generation refuses a present `N/A` value that its reader would discard.
  User-defined values and comments are opaque: blank and `N/A` payloads are retained. Parameter
  names must be nonempty; repeated names preserve ordered entries. XML whitespace around scalar text is trimmed on KVN output and inner TABs become blanks;
  embedded CR/LF remains unrepresentable. Values are trimmed as KVN readers trim them.
  Unit labels tolerate surrounding blanks inside KVN brackets (`[ m ]` reads as `[m]`).
  This lossless lexical extension follows the parsing policy above; ODM 7.7 governs the unit
  value, which is still checked. `bracketed_unit_padding_is_trimmed_before_validation` and the public-message regression
  `bracketed_unit_padding_is_checked_at_public_message_boundaries` cover it.
- **Case:** `EULER_ROT_SEQ` reads mixed-case axis letters in KVN/Python, writing canonical
  uppercase letters; ADM 1.0 numeric sequences (`313`) are rejected; XML requires its exact enum spelling.
  Keywords are uppercase. Closed enumeration values accept all-uppercase or
  all-lowercase spellings and documented aliases, but not arbitrary mixed case. TDM KVN/Python
  closed categories are case-insensitive under TDM 4.3.7; XML retains schema spellings.
  The shared YES/NO KVN/Python lookup is also case-insensitive: this lossless extension
  keeps boolean construction and parsing consistent without family-specific copies of the helper. Free text and
  open or ICD-defined values (frames, time systems, names) are accepted and preserved in any case,
  in every notation; the ODM 7.5.3 single-case rule is not enforced.
- **Underscore:** in KVN/Python categorical lookup, `_` and a blank are interchangeable, and runs of
  blanks collapse to one (ODM 7.5.9, CDM 6.3.2.5, RDM 5.3.3.4, TDM 4.3.8, ADM 6.8.8).
  Writers emit each category's canonical spelling (`ROCKET BODY`, `SINGLE_DIFF`, etc.).
  Free text retains underscores and internal blank runs; normalization is not applied to names.
  XML categories follow their schema enumeration and explicitly documented producer aliases.
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
  fields. CDM's documented producer compatibility accepts empty/nil optional numeric, epoch,
  enum, and text XML leaves only when they carry no meaningful payload; generation omits absent values.

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
  CDM and RDM KVN writers always supply their book-required fixed units, including when an
  optional XML unit annotation was absent.
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
checks each component in `[-1, 1]` and squared norm in `[0.99, 1.01]`. The book specifies no
numeric tolerance; this is an explicit implementation choice accommodating rounded examples,
not a new adjustable parser option. Data outside the interval is rejected rather than normalized.
The common constructor checks component domains; complete-message validation checks magnitude.
KVN generation checks the magnitude again after its 16-digit component rounding and refuses
output that would fail the same validation when read back; this includes ACM maneuver targets.

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

The shared XML input policy follows NDM/XML 3.6.1: application element names are
case-sensitive. Qualified and unqualified schema forms are accepted; foreign namespaces
and mixed element forms are rejected. Missing `xmlns:xsi` is harmless when no prefix uses
it; undeclared prefixes remain invalid. XSI schema hints may use any correctly bound prefix.
`xml:lang`, `xsi:type`, XML 1.1, non-UTF-8 declarations, DOCTYPE, and whitespace before an
XML declaration are refused. Every generated root declares its namespaces. OEM's XML
namespace/root regressions and `library::xml` cover these common rules.

ODM 7.8.5 calls comment whitespace significant, while 7.4.7 says trailing whitespace is
insignificant. The library follows 7.4.7 for fixed-width padding and trims trailing blanks;
leading comment text is preserved. This is an explicit resolution of conflicting clauses.

XML deserializer failures retain structured line, column, byte offset and family context.
Missing XML leaves use `validation.missing_required_field`, the same category as KVN;
field paths and detection locations can differ between notations. Other serde
value-conversion failures may retain schema/type wording and `parse.xml.syntax`;
converting every serde message into a stable semantic category by matching English text
would be brittle. Consumers should use the code and location rather than parse message text.
The lexical source spelling may be normalized in value-error messages (for example `INF`
becomes `inf`); `original_token` is the source excerpt when available.

## OCM XML linkage exception

OCM `validate()` retains the TC1 trajectory/covariance linkage rule. XML parsing and generation
validate all represented content but omit that linkage requirement because NDM/XML 4.0.0 has
no covariance TRAJ_ID element. KVN parsing/generation still enforce it. Populated TC1 fields
that XML cannot represent remain generation errors; no supplied linkage information is dropped.

## Remaining diagnostic boundaries

Invalid UTF-8 retains the `io.error` category and file attributes, with a parse byte offset,
line and column identifying the first invalid byte. UTF-16/UTF-32 remain explicitly refused.

Enum/unit parser failures expose a canonical model field path only when the containing
validator knows it; a lowercased keyword is not substituted for a model path. Some low-level
vector/numeric deserialization errors still have a source location and stable category without
a canonical field path or a field-specific human message. These remain diagnostic limits,
not evidence that validation was bypassed.

## Input detection and diagnostics

File extensions do not select the parser. After a leading UTF-8 BOM and ASCII whitespace,
`<` selects XML; any other nonblank byte selects KVN. Empty input fails. XML family names and
KVN version keywords are case-sensitive. Supplying a notation selects that parser directly;
it does not reinterpret the file extension or turn a document in the other notation into valid
input. Rust `detect_message_header` identifies only the first header/root, whereas
`detect_message_kind` also scans for concatenated KVN headers. Neither validates the body.
Full readers validate the complete message and reject concatenated KVN with
`unsupported.notation`; use a separate file per KVN message.

Empty mandatory values report `validation.missing_required_field`. Invalid epochs may report
`parse.invalid_epoch` at direct constructors, `parse.kvn.syntax` in KVN combinators,
`parse.xml.syntax` in XML deserialization, or `validation.invalid_value` when contextual model
validation finds the error. These codes describe the failing boundary; they do not promise
identical exception classes for every notation. Human messages and low-level type names are not
an API contract. Codes, available canonical paths, and source locations are the machine interface.
Shared structural/envelope errors can lack a location; deserializer detection points identify a
nearby token rather than necessarily its first byte. Combined constituent deserializer locations
are translated back into the original document, and their messages identify the zero-based
constituent index. A leading BOM is excluded from parser offsets after input preparation.

OCM/ACM calendar and relative epoch comparisons recognize equivalent ordinal/calendar
spellings and decimal spellings within one branch, without converting time systems.
Physical plausibility and external algorithms remain caller checks; an omitted XSD cross-field
constraint does not by itself exempt a documented, self-contained book requirement.
