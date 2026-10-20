# TDM 2.0 conformance

This inventory records maintainer evidence for standalone TDM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: TDM 2.0; semantic and KVN authority: CCSDS 503.0-B-2 with
  corrigendum 1 (`data/tdm_503x0b2c1.pdf`).
- XML authority: NDM/XML schema set 4.0.0 and TDM schema 2.0.
- Profile: strict standalone parsing, validation, deterministic KVN/XML generation, and notation
  conversion through the shared Rust generation gate.

## Executable evidence

The TDM suites establish:

| Concern | Evidence |
| --- | --- |
| KVN structure | A local state scanner enforces printable ASCII, the 254-character line limit, header and `META`/`DATA` section sequencing, metadata uniqueness and the `PATH` choice, known observation keywords, and section-scoped comments. Metadata assignments are accepted out of order as a producer-compatibility decision; TDM §3.3.1.8 prescribes table order, which the writer emits. Duplicate assignments and misplaced comments remain errors. |
| XML structure | The shared XML sequence engine covers the root, repeated segments, metadata, data, and the complete observation choice; it rejects unknown, duplicate, reordered, and non-schema nested content while accepting the schema's optional angle/percentage units. |
| Preservation and generation | All 21 shipped KVN fixtures and both shipped XML fixtures preserve their typed source-notation model. Generated XML validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). Fixed optional XML units are semantically normalized when crossing unitless KVN. Bare `COMMENT` records are preserved by the shared KVN writer. |
| History allocation | `tdm_kvn_allocations` compares 10 and 1,000 observations. Parsing allocations grow with vector capacity rather than per record; validated streaming generation has fixed temporary-allocation overhead; materialized storage remains output-proportional. |
| Shared surfaces | `library::parsing` and `library::output`, `conformance::generation`, Python API tests, and the family benchmark matrices exercise shared parsing, generation, diagnostics, and dispatch. |

The existing exhaustive TDM unit tests exercise every observation variant and the indexed metadata
families. `tdm_kvn_history_scaling` provides reproducible Criterion workloads at 100, 10,000 and
50,000 observations and `tdm_xml_history_scaling` at the first two, the sizes shared by every
history-carrying family.

## Complete ICS reconciliation

All 53 numbered features in CCSDS 503.0-B-2 corrigendum 1 annex A2.1.5 were reconciled:

| ICS items | Logical requirement set | Reconciled implementation/evidence |
| --- | --- | --- |
| 1-6 | Header/version fields | `Tdm`, `TdmHeader`, strict root/version/epoch tests |
| 7-16 | Metadata preamble, participants, mode and path choice | `TdmMetadata`, indexed-field and conditional path/mode tests |
| 17-38 | Ephemerides, bands, turnaround, integration/range/angle/interpolation, delays, quality and corrections | complete optional metadata fixture/exhaustive tests, strict known-key scanner, unit normalization, and numeric domains enforced per segment: positive `INTEGRATION_INTERVAL` and `DOPPLER_COUNT_BIAS`, non-negative `RANGE_MODULUS` and all ten indexed transmit/receive delays, and finite `FREQ_OFFSET` and `CORRECTION_*` values |
| 39 | Metadata delimiter | local KVN state scanner and XML segment sequence |
| 40-41 | Data section and comments | strict section scanner and comment preservation tests |
| 42-52 | Every angle, carrier, clock, Doppler, media, meteorological, optical/radar, range, receive, transmit, and VLBI observation family | exhaustive typed observation-choice tests plus all 23 shipped fixtures, and observation value domains enforced through every segment: universal finiteness, `RHUMIDITY` in `[0, 100]`, positive transmit frequencies, non-negative tropospheric delays, `ANGLE_1`/`ANGLE_2` in `[-180, 360)`, and positive `RCS`, `STEC`, and `TEMPERATURE` |
| 53 | Data delimiter | local KVN state scanner and repeated XML segment sequence |

No annex-A TDM feature is absent. TDM's exception to the shared policy on [externally governed values](../design/validation-contract.md#registry-governed-values) is that normative
annex B places time-system and reference-frame registry membership outside the message, so those
values remain caller supplied while local syntax and conditional semantics are enforced.
Concretely, every populated `PATH`, `PATH_1`, and `PATH_2` is re-parsed at
validation and each index is checked against the `PARTICIPANT_n` fields actually supplied, so the
XML and tuple-field routes no longer bypass the path parser; and the `SINGLE_DIFF` requirement for
`RECEIVE_BAND` depends on whether the segment actually carries differenced frequency or range
observations rather than on metadata alone. All six schema spellings of `RANGE_UNITS` are accepted
in XML, while KVN remains intentionally case-insensitive.

## Packaged-surface evidence

`test_tdm.py` and the shared Python options matrix cover both
notations, file IO, epoch rules, generic identity, and diagnostics. Strict binding
audit, generated stubs/doc checks, wheel verification, and Rust artifact verification are the
packaged gates.

## Metadata vocabularies and exact phase counts

Table 3-3 permits ICD-defined `ANGLE_TYPE` values and delegates `REFERENCE_FRAME` to a living
SANA registry. KVN preserves nonempty caller-supplied values in those domains, including frames
outside the older XML enumeration. XML input/output remains constrained to the bundled schema;
conversion refuses an unrepresentable value before output instead of substituting a frame or
angle convention. Known XML enumeration values remain case-sensitive; the KVN reader retains
its documented case aliases. `DATA_TYPES`, when supplied, must contain observation keywords from table 3-5;
indexed families may use the table's `_n` notation. The library does not invent participant
identities, calibration truth, or an ICD for a custom angle convention.

Sections 3.5.2.11–3.5.2.12 define phase counts as real-number strings with any required number of
digits. All ten receive/transmit phase-count variants retain their decimal/exponent text through
KVN, XML, Python, and conversion. They are not parsed through `f64`. Lexical validation rejects
malformed numbers and non-finite spellings without limiting the number of digits. Other
observations use doubles and the ordinary-number generation rules below. The allocation-growth
budget uses ordinary numeric observations; retaining each exact phase count requires storage
proportional to its text.
`tdm::model` tests exact counts above `2**53`, long fractional parts, very large exponents, invalid
lexemes, metadata vocabularies, and zero-byte XML refusals; Python tests exercise all ten variants.
The exact decimal model and XML do not impose a precision limit. KVN still applies its separate
254-character line limit: a phase-count record that exceeds it is refused, never truncated.

For ordinary numeric observations and metadata, TDM sections 4.3.4–4.3.5 limit fixed-point
digits and floating-point mantissas to 16 digits. KVN generation uses the shared numeric
formatter to round when necessary; values whose rounded spelling would overflow a double
are refused before output. XML retains the original double value. The bundled TDM XSD
types ordinary observations and phase counts as doubles, but the more specific book
sections 3.5.2.11–3.5.2.12 require arbitrary-precision phase-count strings. Those counts keep
their validated decimal text rather than passing through the ordinary-number formatter.
`tdm::generation::kvn_numbers_follow_the_digit_limit_without_rounding_phase_counts` and
`kvn_refuses_numbers_whose_rounded_spelling_overflows_before_output` cover both boundaries.

## Verification outcome

TDM 2.0 meets the common [verification criteria](README.md#verification-criteria) for
self-contained format and model validation on Rust and Python. The requirement inventory,
family tests, shared public-route tests, and packaged tests are complementary evidence;
none alone establishes conformance or exhaustive mutation coverage.

## Decisions and limits

**Observation order is per keyword.** Sections 3.4.10–3.4.11 require strictly increasing,
non-repeated epochs for each observation keyword within a segment. Different keywords may
interleave in any order, including a decrease in the overall row sequence. Validation checks
each keyword separately and generation preserves producer row order; it never sorts records.
Indexed receive/transmit frequency, frequency-rate, and phase-count observations require the
corresponding `PARTICIPANT_n`, just as PATH references require a populated participant.
`epochs::observations_are_ordered_per_keyword_without_reordering_other_keywords` and
`model::doppler_counts_are_integral_and_indexed_observations_require_their_participant` cover
these rules through parsing and generation.

The published Annex E17 example repeats the RCS epoch `2011-05-11T10:26:33.7008` for two
different measurements, contrary to section 3.4.11. Its original fixture is retained unchanged
as `data/invalid/tdm_e17_original.kvn` and rejected. The annotated positive fixture adapts the
last RCS epoch to `2011-05-11T10:26:33.9686`, matching the other measurements in that group.
That repair is a test-fixture choice, not a correction published by CCSDS or a parser rewrite.
`parsing::original_e17_repeated_rcs_epoch_is_rejected_and_adaptation_remains_valid` covers both.

**Doppler counts are integral doubles.** Section 3.5.2.4 requires `DOPPLER_COUNT` to be an
integer, although the XML schema declares an unrestricted double. The public numeric type
remains `f64`; fractional, non-finite counts are rejected on both notations and generation.
Positive counts are only recommended, so signed integer counts and zero remain accepted.
This count is distinct from the arbitrary-precision decimal phase-count strings described
above. Values already rounded by a caller into a double cannot recover lost integer digits.

**Epoch storage has a library limit.** All epoch tokens, including observation epochs, are
limited to 64 bytes before parsing. This is not a CCSDS or XSD length restriction. Longer
tokens are rejected, never truncated, on both notations and in direct epoch construction.
Other field-specific precision rules still apply where documented. See the shared
[epoch storage policy](../design/validation-contract.md#epoch-storage-limit).

The current implementation and evidence cover edition 2.0. Previous acceptance of 1.0
merely reused the 2.0 grammar; tests changed version labels without providing actual 1.0
wire-syntax and metadata fixtures. Agreement of several numeric domains across editions
does not establish edition support. Input 1.0 is rejected until a complete edition-specific
review and implementation is available.

Time-system and frame registry membership, calibration truth, and the correctness of an
exchange-partner path are outside the message. Local participant-index existence, path/mode
choices, observation domains, and conditional metadata remain library checks.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

All four book line endings (LF, CR, CRLF and LFCR) are accepted with identical models and
byte-position-preserving normalization.

Optional XML `ANGLE_1`/`ANGLE_2` unit annotations are checked against the schema's `deg`
spelling; `rad` is rejected rather than ignored or converted. Their fixed degree annotation
may be omitted by the writer. Ordinary observation numbers use the shared finite-number reader,
including nonzero-underflow rejection; exact phase counts retain their separate decimal model.
`tdm_xml_accepts_the_schema_defined_optional_observation_units` covers incompatible annotations,
and `observation_numbers_use_shared_reading_rules_without_underflow` covers observation spelling
and range boundaries.
