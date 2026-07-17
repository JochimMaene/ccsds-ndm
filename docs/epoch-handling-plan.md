# Epoch handling architecture

Status: implemented foundation; context-aware physical-time conversion remains deferred.

This document records the current design and its remaining decisions. It is not a conformance
claim. Exact support remains defined by the [support matrix](support-matrix.md).

## Authority

- The XSDs in `data/xsd/` define XML wire structure and lexical types.
- The applicable books in `docs/ccsds-books/` define field meaning and semantic restrictions not
  represented by the XSD.
- A broad XSD type does not override a stricter field rule in a book.

The shared `epochType` is a union of:

- a calendar or ordinal time string; and
- a signed decimal token.

The token alone does not always identify a physical instant. `TIME_SYSTEM`, `EPOCH_TZERO`,
MET/MRT reference events, SCLK parameters, or an ICD can be required.

## Current representation

```text
Epoch             XSD-facing calendar/numeric union
CalendarEpoch     validated calendar/ordinal view for fields proven to require that branch
RelativeTime      ADM/ACM relTimeType spelling (xsd:double), distinct from epochType
```

`Epoch` stores:

- the original spelling in a 64-byte inline buffer;
- its length; and
- its calendar/numeric classification.

Its current size is 66 bytes. It does not retain an ordering cache.

`CalendarEpoch` is a transparent wrapper around `Epoch`, so narrowing a field adds no storage.
There is no separate `RelativeEpoch` type: no message field currently needs that public invariant.
Contextual numeric time tags remain the numeric branch of `Epoch`.

`RelativeTime` exists because `relTimeType` is `xsd:double`, whose fixed/scientific lexical rules
are different from the decimal branch of `epochType`.

### Resource boundary

Epoch and relative-time spellings are limited to 64 bytes. Inputs beyond the limit return a
length-only error and are not copied into diagnostics. This is an implementation resource bound,
not an expansion or normalization of the CCSDS lexical space.

Serde XML deserialization still creates a temporary `String`. Replacing that with a borrowed
visitor is optional optimization work and requires a measured benefit; it is not needed for the
model invariant.

## Validation and comparison

Construction classifies the XSD branch and validates calendar fields once. Contextual message
validation rejects malformed calendar values and degenerate numeric spellings.

Same-branch ordering is exact and allocation-free:

- decimal values are compared without converting through `f64`;
- calendar and ordinal dates share a proleptic-Gregorian day key;
- fractional seconds retain arbitrary supported precision;
- leap-second ordering is retained.

Comparison state is derived during the validation pass instead of being stored in every epoch.
This trades a small amount of validation CPU for a smaller persistent model and simpler invariants.
The XSD's timezone-offset spellings remain representable by the wire-level `Epoch`, but strict
calendar fields reject them because ODM §7.5.10 and ADM §6.8.9 permit only an optional `Z`.

Current OCM behavior:

- trajectory and covariance blocks require one epoch branch, unique tags, and strictly increasing
  tags within each block;
- maneuver blocks require the branch selected by `MAN_COMPOSITION` and reject duplicate tags, but
  do not invent a monotonic-order rule;
- no comparison is made across independent blocks.

The detailed contract and normative citations are in
[`conformance/time-ordering-contract.md`](conformance/time-ordering-contract.md).

## Field migration

Fields proven by the books to require calendar/ordinal spelling use `CalendarEpoch`. Contextual
fields retain `Epoch`.

Completed inventories:

- [OPM](conformance/epoch-field-inventory-opm.md)
- [OMM](conformance/epoch-field-inventory-omm.md)
- [OEM and OCM](conformance/epoch-field-inventory-oem-ocm.md)
- [ADM header](conformance/epoch-field-inventory-adm.md)
- [APM and AEM](conformance/epoch-field-inventory-apm-aem.md)
- [ACM and OCM](conformance/epoch-field-inventory-acm-ocm.md)
- [CDM, TDM, and RDM](conformance/epoch-field-inventory-cdm-tdm-rdm.md)

Rust field narrowing is intentionally breaking while the library API is still being established.
Python retains string properties but validates through the same Rust types.

## OCM context-free semantic coverage

The current OCM slice additionally enforces rules that do not require physical-time resolution:

- `TIME_SYSTEM=SCLK` requires both SCLK reference parameters;
- SCLK/time-offset values use the XSD seconds unit;
- `NEXT_LEAP_EPOCH` requires `NEXT_LEAP_TAIMUTC`;
- duty-cycle type selects its required fields;
- `TIME_AND_ANGLE` requires its angle and trigger fields;
- durations are finite, non-negative seconds;
- pulse period is not shorter than pulse duration;
- `DC_MAX_CYCLES` is positive;
- phase angles use the XSD range;
- the explicit execution/window inequalities are checked where both operands use a comparable
  branch.

Physical SCLK, MET/MRT, and unknown-ICD resolution is not claimed.

## Performance evidence

Local measurements on 2026-07-16/17 used Rust 1.90.0 and an Intel i7-1165G7. They are development
baselines, not portable guarantees.

| Operation | Measured range |
| --- | --- |
| Parse 10k-state OEM XML | 29.818–33.388 ms |
| Generate 10k-state OEM XML | 14.478–15.165 ms |
| Validate 10k-state OEM | 64.928–82.727 µs |
| Validate 10k-record OCM trajectory | 1.162–1.337 ms |
| Validate 10k-record OCM covariance | 1.105–1.241 ms |
| Validate 10k-record OCM maneuver | 0.542–0.623 ms |
| CCSDS calendar semantic check, 10k values | 0.225–0.243 ms |
| `hifitime` 4.3 parse, 10k values | 3.677–4.058 ms |

The `hifitime` experiment was around six times slower than lexical parsing and rejected valid
CCSDS ordinal, numeric, and high-precision forms. It is therefore not a wire parser or storage
type.

Permanent OEM/OCM parse, generation, validation, and scaling cases live in:

- `ccsds-ndm/benches/xml_benches.rs`
- `ccsds-ndm/benches/kvn_benches.rs`
- `ccsds-ndm/examples/ocm_memory.rs`

The 88-byte cached `Epoch` prototype was removed after review. It made isolated OCM ordering
validation tens of microseconds, but added 22 persistent bytes to every epoch, including fields
that are never ordered. The consolidated 66-byte representation derives one private ordering key
per record and retains only the previous key during validation. For 10,000 records this avoids
about 220 KB of persistent model storage at a measured validation cost of at most roughly 1.3 ms.
This is the preferred tradeoff unless a published end-to-end budget demonstrates otherwise.

## Deferred physical-time API

Do not add `TimeContext` merely to represent metadata already present in the message. Introduce it
only with a concrete operation that cannot be implemented correctly without physical-time
resolution.

Any future context must be borrowed and preserve:

- the original `TIME_SYSTEM`, including unknown ICD-defined values;
- `EPOCH_TZERO`;
- optional SCLK offset/rate parameters;
- caller-supplied MET/MRT reference events;
- message-provided leap-second/EOP values without silently replacing them with a library table.

Do not copy that context or a resolved physical epoch into each history record.

`hifitime` may later be an explicit conversion adapter. It must not become the wire model, and its
precision/time-system limitations must remain visible to callers.

## Next step

Do not expand epoch conversion further. First:

1. remeasure the compact representation;
2. keep the field inventories and regression tests green;
3. close the first support-matrix tracer bullet for OPM 3.0 XML generation.

Only then select a concrete context-aware comparison or arithmetic consumer. If no library feature
needs physical-time arithmetic, stop without adding `TimeContext` or a time-library dependency.
