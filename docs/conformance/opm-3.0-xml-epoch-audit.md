# OPM 3.0 XML epoch audit

Scope: `OPM-3.0-XML-GENERATE-RUST`.

The NDM/XML 4.0.0 XSD declares these values with the broad `epochType` union. CCSDS
502.0-B-3 §§3.2.3.2 and 7.5.10–7.5.11 supply the field semantics and the stricter
calendar/ordinal spelling.

## Fields

| Field | Book meaning | Rust boundary |
| --- | --- | --- |
| `header.creation_date` | Mandatory UTC creation time | `CalendarEpoch` |
| `metadata.ref_frame_epoch` | Optional frame-definition epoch | `Option<CalendarEpoch>` |
| `data.state_vector.epoch` | State epoch under metadata `TIME_SYSTEM` | `CalendarEpoch` |
| `maneuver_parameters[*].man_epoch_ignition` | Maneuver epoch under metadata `TIME_SYSTEM` | `CalendarEpoch` |

`TIME_SYSTEM` changes interpretation, including MET/MRT rules, but OPM still refers these fields to
the §7.5.10 calendar/ordinal syntax. The lexical type therefore does not perform physical-time
conversion.

## Enforced invariants

`CalendarEpoch` preserves the original spelling and rejects:

- the numeric branch of the XSD union;
- impossible calendar and ordinal fields;
- empty fractional parts;
- negative or extended years, because the book specifies exactly `YYYY`;
- timezone offsets, because §7.5.10 permits only the optional `Z` terminator; and
- tokens longer than the library's published 64-byte epoch bound.

The underlying `Epoch` union remains able to represent both XSD branches for fields whose legal
branch is contextual. A broad schema type therefore does not weaken a stricter OPM field rule.

Focused Rust parsing, public-mutation, and generation tests are in
[`opm_epoch_xml_generation.rs`](../../ccsds-ndm/tests/opm_epoch_xml_generation.rs).

## Caller-context boundary

Self-contained validation does not claim to establish:

- the external reference event needed to resolve MET/MRT;
- an ICD-defined time system or frame;
- whether a `:60` value coincides with a real leap-second introduction; or
- the factual correctness of a supplied epoch.

Those checks require explicit caller context. They must not be guessed or silently replaced by a
library time table.
