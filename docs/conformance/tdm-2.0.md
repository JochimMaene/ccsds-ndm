# TDM 2.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone TDM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: TDM 2.0; semantic and KVN authority: CCSDS 503.0-B-2.
- XML authority: NDM/XML schema set 4.0.0 and TDM schema 2.0.
- Profile: strict standalone parsing, validation, deterministic KVN/XML generation, and notation
  conversion through the shared Rust generation gate.

## Executable evidence

`just conformance-tdm` establishes:

| Concern | Evidence |
| --- | --- |
| KVN structure | A local state scanner enforces printable ASCII, the 254-character line limit, header and `META`/`DATA` section sequencing, metadata uniqueness and the `PATH` choice, known observation keywords, and section-scoped comments. It deliberately does not impose XML metadata ordering on order-independent TDM KVN. |
| XML structure | The shared XML sequence engine covers the root, repeated segments, metadata, data, and the complete observation choice; it rejects unknown, duplicate, reordered, and non-schema nested content while accepting the schema's optional angle/percentage units. |
| Preservation and generation | All 21 shipped KVN fixtures and both shipped XML fixtures preserve their typed source-notation model. Generated XML validates against the official 4.0.0 master schema. Schema validation runs through libxml2, which establishes structure, ordering, and lexical form; it is not evidence of numeric domain validity, because libxml2 accepts NaN against bounding facets (see the XSD oracle policy in the [validation contract](../design/validation-contract.md)). Fixed optional XML units are semantically normalized when crossing unitless KVN. Bare `COMMENT` records are preserved by the shared KVN writer. |
| History allocation | `tdm_kvn_allocations` compares 10 and 1,000 observations. Parsing allocations grow with vector capacity rather than per record; validated streaming generation has fixed temporary-allocation overhead; materialized storage remains output-proportional. |
| Shared surfaces and limits | `family_contract`, `family_generation_evidence`, Python option tests, and the family benchmark matrices exercise shared bounded parsing/generation, diagnostics, and dispatch. |

The existing exhaustive TDM unit tests exercise every observation variant and the indexed metadata
families. `tdm_kvn_history_scaling` (100-50,000 observations) and `tdm_xml_history_scaling`
(100-10,000 observations) provide reproducible Criterion workloads.

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

No annex-A TDM feature is absent. TDM's exception to the shared policy on [externally governed values](family-shared-contract.md#externally-governed-values) is that normative
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
notations, file IO, epoch rules, generic identity, diagnostics, and resource limits. Strict binding
audit, generated stubs/doc checks, wheel verification, and Rust artifact verification are the
packaged gates.

## Recorded Criterion observations

The registered history workloads were run on 2026-07-18 with 20 samples, a 0.5 s warm-up, and a
1 s target measurement time:

```text
cargo bench --manifest-path ccsds-ndm/Cargo.toml --bench kvn_benches -- 'tdm_kvn_history_scaling' --sample-size 20 --measurement-time 1 --warm-up-time 0.5
cargo bench --manifest-path ccsds-ndm/Cargo.toml --bench xml_benches -- 'tdm_xml_history_scaling' --sample-size 20 --measurement-time 1 --warm-up-time 0.5
```

The observed Criterion 95% time intervals were:

| Notation/path | 100 records | 1,000 records | 10,000 records | 50,000 records |
| --- | --- | --- | --- | --- |
| KVN parse | 45.373-52.422 us | 391.26-411.11 us | 4.0251-4.4088 ms | 20.266-21.816 ms |
| KVN generate | 14.725-16.980 us | 120.95-125.71 us | 1.2416-1.3050 ms | 6.5180-7.2676 ms |
| XML parse | 131.83-148.38 us | 1.2382-1.3194 ms | 12.481-13.709 ms | not registered |
| XML generate | 40.812-45.052 us | 395.03-421.06 us | 4.5055-5.1199 ms | not registered |

These observations show approximately record-proportional scaling over the registered ranges.
They are reproducible comparison evidence, not release thresholds or cross-machine performance
claims.

## Remaining verification work

TDM remains `implemented-unverified` under the [shared promotion policy](family-shared-contract.md#promotion-policy). Its ICS feature inventory and packaged
surfaces are reconciled, and the recorded timing observations below remain informational.

Both accepted input editions are covered for value domains. The observation value domains above are enforced
regardless of edition, and 503.0-B-1 was checked directly to confirm that is correct: it specifies
the same `-180.0 <= ANGLE_1 < 360.0` bound and the same positive `TEMPERATURE` and `STEC`, and it
has no `RCS` keyword at all, so that rule is vacuous rather than conflicting for 1.0 input. No
edition-conditional rule is required.

That check is not full 1.0 edition support. What remains unverified for 1.0 is wire-syntax and
metadata parsing specifically: no shipped 1.0 fixture exercises 1.0-only KVN/XML syntax or metadata
paths through the Rust and Python parsers. Value-domain agreement is verified; 1.0 syntax/metadata
coverage is the remaining named proof before any 1.0 support claim.
