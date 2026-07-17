# Time-ordering and duplicate-time contract

Status: design and evidence plan. This document does not advertise conformance and does not
authorize a public API change.

## Authority

The official XSDs in `data/xsd/` decide which time-token spellings can appear on the wire. The
applicable CCSDS books in `docs/ccsds-books/` decide whether a field is absolute or relative and
which records must be ordered or must not repeat. The comparator must never infer a physical time
system from the token alone.

## Normative rules to implement

| Family / block | Normative rule | Source | Comparison scope |
| --- | --- | --- | --- |
| OCM `TRAJ` | No duplicate time tags; history is monotonically increasing | ODM §§6.2.2.4, 6.2.5.6 | One trajectory block only |
| OCM `COV` | No duplicate time tags; history is monotonically increasing | ODM §§6.2.2.4, 6.2.7.6 | One covariance block only |
| OCM `MAN` | No duplicate time tags; one `TIME_ABSOLUTE` or `TIME_RELATIVE` branch per block | ODM §§6.2.2.4, 6.2.2.5, 6.2.8.18 | One maneuver block only; no monotonic rule is stated |
| AEM attitude data | Increasing time and no repeated time tags | ADM §4.2.4.8.1 | One AEM data block |
| ACM all data blocks | No duplicate time tags; one branch per block | ADM §§5.3.4.4–5.3.4.5 | Each ACM data block; sections may match each other |
| ACM `COV` | Covariance history is monotonically increasing | ADM §5.3.7.5 | One covariance block |
| OEM ephemeris | Repeated tags are allowed across consecutive blocks under the span rule; no intra-block monotonic rule is stated | ODM §5.2.4.4 | Do not add a stricter global order check |
| TDM observations | No global record order; records for a given keyword may be chronological | TDM §3.4.10 | Per observation keyword when a caller requests it |

The OCM validator now covers the table's OCM history rules for same-branch values. It validates
calendar/numeric branch consistency, rejects numerically equal spellings such as `1`/`1.0`, and
rejects duplicate or decreasing trajectory/covariance tags. Maneuver blocks reject duplicates but
do not impose an ordering rule that the book does not state. Comparison state is derived only
during the validation pass; it is not retained in every wire value.

The OCM maneuver validator also enforces the self-contained duty-cycle rules in ODM
§§6.2.8.20.6–6.2.8.20.8: non-continuous maneuvers require the seven window, execution, reference,
duration, and period fields; `TIME_AND_ANGLE` additionally requires its five angle/trigger fields;
and a pulse period cannot be shorter than its pulse duration. Pulse durations and periods are
validated as finite, non-negative seconds, matching the XSD's `durationType` (`timeUnits="s"`).
It also preserves the XSD's positive-integer constraint for `DC_MAX_CYCLES` and angle range for
the optional phase-angle fields when callers mutate a typed model.
The optional maneuver epoch fields are required to use the same absolute/relative branch selected
by `MAN_COMPOSITION`, as required by ODM §6.2.2.5. The two explicit schedule inequalities in the
ODM table are also enforced: `DC_EXEC_START >= DC_WIN_OPEN` and `DC_EXEC_STOP <= DC_WIN_CLOSE`.
Other cross-field ordering (for example `START_TIME`/`STOP_TIME` or usable-span containment) is
deferred until it needs the same borrowed time context as span comparisons.

OCM metadata checks independently enforce that `TIME_SYSTEM=SCLK` has both SCLK parameters and
that `NEXT_LEAP_EPOCH` has `NEXT_LEAP_TAIMUTC`; these are presence and wire-unit rules, not a
physical-time conversion claim.

## Comparison contract

An ordering check operates on two already-validated `Epoch` values and returns one of:

- `Less`, `Equal`, or `Greater` when both values can be resolved under the same interpretation;
- `Indeterminate` when a required interpretation or external reference is missing.

`Equal` means the same represented time, not merely identical spelling. For example, `1`, `+1`,
and `1.0` are equal numeric time tags. A comparator must not convert arbitrary-precision CCSDS
fractions through `f64` before deciding equality.

The internal comparator is allocation-free and used by one validation pass. It compares
same-branch OCM history values without a message-wide physical-time cache, and handles:

- book-valid calendar and ordinal dates, leap-second `ss=60`, optional `Z`, and arbitrary
  fractional precision;
- signed decimal relative values without losing significant digits; and
- the exact branch and field context selected by `MAN_COMPOSITION`.

Cross-branch and cross-field checks remain context-dependent. `START_TIME`/`STOP_TIME`, usable-span
containment, duty-cycle windows, MET/MRT reference events, SCLK parameters, and unknown ICD time
systems must return `Indeterminate` until the caller supplies a borrowed context containing the
needed reference information. Strict parsing must not guess or silently skip a required check.

## Context and API boundary

`TimeContext` remains a borrowed semantic view, not a field copied into every history record. It
will be introduced only with a comparator consumer and a measured performance budget. The wire
model continues to retain the original `Epoch` spelling. `hifitime` remains an optional conversion
adapter, not a parser or storage type.

## Evidence gates for implementation

Before exposing or advertising an ordering check, add fixtures for:

1. increasing, equal-by-value, and decreasing numeric tags;
2. calendar and ordinal forms, fractional precision, leap seconds, and year boundaries;
3. mixed branches, missing context, MET/MRT, SCLK, and unknown time-system values; and
4. the exact OCM/AEM/ACM block boundaries in the table above.

The 10k OCM trajectory, covariance, and maneuver benchmark groups measure validation with the
comparator. The successful path allocates nothing. `Epoch` retains only its spelling and
classification (66 bytes); comparison work is transient, so records that are never ordered do not
pay a persistent cache cost. XSD timezone-offset spellings are rejected before comparison under
the stricter book-level calendar-field rules.
