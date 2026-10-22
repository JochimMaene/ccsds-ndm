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
| KVN structure | A local state scanner enforces printable ASCII, header and `META`/`DATA` section sequencing, metadata uniqueness and the `PATH` choice, known observation keywords, and section-scoped comments. Metadata assignments are accepted out of order as a producer-compatibility decision; TDM §3.3.1.8 prescribes table order, which the writer emits. The four header keywords, by contrast, must keep table 3-2 order: the header is short, fixed, and no known producer reorders it, so no tolerance was added. Duplicate assignments and misplaced comments remain errors. |
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
observations rather than on metadata alone. All six schema spellings of `RANGE_UNITS` (`km`, `KM`, `s`, `S`, `RU`, `ru`) are accepted
in XML. KVN reads `RANGE_UNITS` in any letter case (`Km`, `rU`): table 3-3 lists the three values
without a case rule, and each case variant names exactly one unit. Every writer emits the book
spellings `km`, `s`, and `RU`. This is a TDM alias set in the sense of the shared
[case rule](../design/validation-contract.md#shared-lexical-decisions).

## Packaged-surface evidence

`test_tdm.py` and the shared Python options matrix cover both
notations, file IO, epoch rules, generic identity, and diagnostics. Strict binding
audit, generated stubs/doc checks, wheel verification, and Rust artifact verification are the
packaged gates.

## Metadata vocabularies and exact phase counts

Table 3-3 permits ICD-defined `ANGLE_TYPE` values and delegates `REFERENCE_FRAME` to a living
SANA registry. KVN preserves nonempty caller-supplied values in those domains, including frames
outside the older XML enumeration. XML input/output uses the bundled schema's value domains;
conversion refuses an unrepresentable value before output instead of substituting a frame or
angle convention. Known XML enumeration values accept the uppercase/lowercase forms listed by
the XSD. Compatibility frame aliases `ITRF-2000`, `ITRF1993`, `ITRF93`, and `TOD_EARTH` are also
accepted in XML and generate as `ITRF2000`, `ITRF-93`, `ITRF-93`, and `TOD`, respectively.
Arbitrary mixed case is rejected in XML. KVN reads the same aliases and the book spellings
`ITRF1993` and `TOD_EARTH` as the known frames, so KVN output is canonical too: `ITRF1993`
and `ITRF93` are written `ITRF-93`, `ITRF-2000` is written `ITRF2000`, and `TOD_EARTH` is
written `TOD`. Only values outside the known frames (`GCRF`, mixed-case spellings) are kept as
written in KVN.

`REFERENCE_FRAME` is required whenever `ANGLE_TYPE` is `RADEC`, in any letter case (a mixed-case
`Radec` is kept as an open value but still triggers the rule). Table 3-3 says the frame "shall
be used in conjunction with" RADEC, and right ascension/declination angles cannot be interpreted
without their inertial frame; the XSD does not express the dependency. `DATA_TYPES`, when supplied, must contain observation keywords from table 3-5;
indexed families may use the table's `_n` notation. The library does not invent participant
identities, calibration truth, or an ICD for a custom angle convention.

Sections 3.5.2.11–3.5.2.12 define phase counts as real-number strings with any required number of
digits, and 4.3.11 adds that the KVN string "must not contain any alphabetic or special
characters". The XSD types the same elements as `xsd:double`, whose lexical space includes
exponents. Both authorities are kept per notation: KVN input rejects an exponent (`1.5E3`) in a
phase count; XML input accepts and retains it, trimmed of surrounding blanks like every XML
number. Such a count cannot be written to KVN without rewriting its digits, so KVN generation and
XML-to-KVN conversion refuse it, naming the keyword, rather than expanding the exponent.
`parsing::phase_count_exponents_are_xml_double_spellings_but_not_kvn_phase_counts` covers all
three paths. All ten receive/transmit phase-count variants retain their source text in the model,
XML and Python. Decimal counts also preserve it through KVN and conversion. They are not parsed
through `f64`. Lexical validation rejects
malformed numbers and non-finite spellings without limiting the number of digits. Other
observations use doubles and the ordinary-number generation rules below. The allocation-growth
budget uses ordinary numeric observations; retaining each exact phase count requires storage
proportional to its text.
`tdm::model` tests exact counts above `2**53`, long fractional parts, very large exponents, invalid
lexemes, metadata vocabularies, and zero-byte XML refusals; Python tests exercise all ten variants.
The exact decimal model and XML do not impose a precision limit. KVN output still applies its separate
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
Indexed phase counts (`RECEIVE_PHASE_CT_n`, `TRANSMIT_PHASE_CT_n`) require the corresponding
`PARTICIPANT_n`, because §§3.5.2.11–3.5.2.12 index them "to enable association with the
PARTICIPANT_n", just as PATH references require a populated participant. The indexed
frequencies are not tied to a participant: §3.5.2.8 indexes `RECEIVE_FREQ_n` by downlink and
§3.5.2.9 indexes `TRANSMIT_FREQ_n` (and so `TRANSMIT_FREQ_RATE_n`) by transmitter, so they need no
`PARTICIPANT_n`. A participant or `RECEIVE_BAND` whose value is blank (an empty XML element) does
not count as populated for the PATH, phase-count, or `SINGLE_DIFF` rules.
`epochs::observations_are_ordered_per_keyword_without_reordering_other_keywords` and
`model::doppler_counts_are_integral_and_only_phase_counts_require_their_participant` cover
these rules through parsing and generation.

`PATH`, `PATH_1`, and `PATH_2` are comma lists with "no inserted white space" (table 3-3). KVN
input with blanks around the commas (`PATH = 2, 1`, as Orekit writes it) is read as the canonical
`2,1`; the blanks carry no information, so this is a lossless producer tolerance under the
[shared policy](../design/validation-contract.md#authority-and-representability). Writers emit
`2,1`. XML follows the schema pattern `\d,\d(,\d)*` and model values must already be canonical.
`parsing::kvn_paths_with_blanks_after_commas_read_as_canonical_paths` covers it.

A lone `PATH_1` or `PATH_2` is rejected in every mode, although the XSD makes each element
optional on its own. Table 3-3 defines the pair as the two legs of differenced (`SINGLE_DIFF`)
data, and a single signal path is exactly what `PATH` expresses, so a lone leg describes no
signal path the book defines.

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

**Epoch spellings follow the book.** XML epochs with a timezone offset, bare numeric epochs, and
empty epochs are rejected although the XSD `epochType` admits them; this is the shared
book-over-XSD decision stated once in the
[validation contract](../design/validation-contract.md#authority-and-representability).

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

## Stricter than XSD

These TDM 2.0 book domains are narrower than the bundled TDM 2.0 XSD. The book rule applies in
KVN, XML, and model validation alike, so a valid model always converts:

| Field | Book (503.0-B-2) | XSD (`ndmxml-4.0.0-tdm-2.0.xsd`) |
| --- | --- | --- |
| `ANGLE_1`, `ANGLE_2` | §§3.5.4.2–3.5.4.3: `-180 <= angle < 360` | `ndm:angleType`: `[-360, 360)` |
| `RCS` | §3.5.5.2: positive | `xsd:double` |
| `STEC` | §3.5.7.1: positive | `xsd:double` |
| `TEMPERATURE` | §3.5.8.3: positive | `ndm:nonNegativeDouble` (admits 0) |
| `DOPPLER_COUNT` | §3.5.2.4: integer | `xsd:double` |
| KVN phase counts | §4.3.11: no alphabetic characters | `xsd:double` (exponents allowed; kept in XML, see above) |

Where book and XSD agree the bound also applies in every notation, under the shared
[facet rule](../design/validation-contract.md#authority-and-representability): positive
`TRANSMIT_FREQ_n` (§3.5.2.9 and `ndm:positiveDouble`),
`RHUMIDITY` in `[0, 100]` (`ndm:percentageTypeUO`; the range error names `RHUMIDITY`), non-negative `TROPO_DRY`/`TROPO_WET` (`ndm:nonNegativeDouble`), and positive
`INTERPOLATION_DEGREE` and `DOPPLER_COUNT_SCALE` (`xsd:positiveInteger`).
`generation::tdm_observation_domains_follow_the_narrower_book_range` and
`generation::unambiguous_tdm_observation_domains_are_revalidated` cover them.

## KVN line length

TDM 4.2.1 limits a KVN line to 254 characters and defines no continuation lines. Longer lines are
read (Orekit writes a 708-character `DATA_TYPES` line), as the
[shared lexical rules](../design/validation-contract.md#shared-lexical-decisions) allow. KVN
generation refuses any `KEY = value` record or phase-count observation that would exceed the
limit, naming the keyword (for example `DATA_TYPES`); it never wraps or truncates. XML has no line
limit, so such a message still converts to XML.
`generation::kvn_output_refuses_integers_and_lines_beyond_the_kvn_limits` covers it.

## Observation units

Optional XML `ANGLE_1`/`ANGLE_2` unit annotations are checked against the schema's `deg`
spelling; `rad` is rejected rather than ignored or converted. Their fixed degree annotation
may be omitted by the writer. Ordinary observation numbers use the shared finite-number reader,
including nonzero-underflow rejection; exact phase counts retain their separate decimal model.
`tdm_xml_accepts_the_schema_defined_optional_observation_units` covers incompatible annotations,
and `observation_numbers_use_shared_reading_rules_without_underflow` covers observation spelling
and range boundaries.

## XML default elements and numeric storage

The bundled TDM schema assigns zero defaults to present, empty `FREQ_OFFSET`, `RANGE_MODULUS`,
all five `TRANSMIT_DELAY_*` and all five `RECEIVE_DELAY_*` elements, and observation
`CLOCK_BIAS`/`CLOCK_DRIFT`. Empty `DATA_QUALITY` means `RAW`. Readers materialize those
values; writers emit them explicitly. Omitted optional metadata remains absent rather
than being populated. Empty elements without a schema default remain invalid.
`tdm::parsing::present_empty_xml_elements_use_schema_defaults_and_omissions_stay_absent`
checks every default-bearing field, both empty element spellings, and nearby invalid cases.

The [integer storage limits](../design/validation-contract.md#integer-storage-limits) apply:
`INTERPOLATION_DEGREE` is positive `u32` and `DOPPLER_COUNT_SCALE` is positive `u64`,
although their XSD positive-integer types have no upper bound. KVN integers are limited to
2³¹−1 by TDM 4.3.2, so KVN generation refuses either value above
`i32::MAX`, naming the keyword; XML keeps the full storage range. Exact phase counts remain
strings. Schema-listed lowercase metadata forms are covered by
`tdm::parsing::xml_accepts_schema_listed_lowercase_categories`.
