# OEM and OCM epoch field inventory

Status: architecture input, not a conformance claim. Scope: OEM 3.0 and OCM 3.0.

## Interpretation rules

The XML schema is authoritative for XML shape and lexical types. All scalar fields below use the
shared `ndm:epochType`, whose lexical alternatives are a calendar/ordinal time code or a signed
decimal. OCM history lines are instead `xsd:string`; their internal time-tag grammar comes from the
ODM book.

The ODM book then supplies these semantics:

- section 7.5.10 defines calendar and ordinal spellings for absolute time tags;
- section 7.5.11 fixes `CREATION_DATE` to UTC and gives other epochs the metadata `TIME_SYSTEM`;
- section 3.2.3.2 makes MET/MRT values durations from an externally documented reference event;
- OCM section 6.2.4 makes `EPOCH_TZERO` the reference for relative data-block times and adds SCLK
  parameters when `TIME_SYSTEM=SCLK`;
- OCM maneuver composition explicitly chooses `TIME_ABSOLUTE` or `TIME_RELATIVE`.

“Contextual” below means the wire form cannot be converted to a physical instant using the field
alone. It needs `TIME_SYSTEM`, `EPOCH_TZERO`, a MET/MRT reference event, SCLK parameters, or an ICD.

## OEM inventory

| Location / XML name | XSD occurrence | Book meaning | Current representation | Python exposure | Proposed category |
| --- | --- | --- | --- | --- | --- |
| `header.creation_date` / `CREATION_DATE` | required `epochType` through the common ODM header | Absolute UTC creation time | `CalendarEpoch` | `str` | `CalendarEpoch` |
| `metadata.ref_frame_epoch` / `REF_FRAME_EPOCH` | optional `epochType` | Absolute epoch when the frame definition needs one; selected time system | `Option<CalendarEpoch>` | `str | None` | `CalendarEpoch` plus segment context |
| `metadata.start_time` / `START_TIME` | required `epochType` | Start of the total ephemeris/covariance span; interpretation follows `TIME_SYSTEM`, including MET/MRT rules | `Epoch` | `str` | contextual `Epoch` |
| `metadata.useable_start_time` / `USEABLE_START_TIME` | optional `epochType` | Start of the usable ephemeris span under the segment time system | `Option<Epoch>` | `str | None` | contextual `Epoch` |
| `metadata.useable_stop_time` / `USEABLE_STOP_TIME` | optional `epochType` | End of the usable ephemeris span under the segment time system | `Option<Epoch>` | `str | None` | contextual `Epoch` |
| `metadata.stop_time` / `STOP_TIME` | required `epochType` | End of the total ephemeris/covariance span; interpretation follows `TIME_SYSTEM` | `Epoch` | `str` | contextual `Epoch` |
| `data.state_vector[*].epoch` / `stateVector/EPOCH` | one required `epochType` per required `stateVector` | State time tag under the segment time system | `StateVectorAcc.epoch: Epoch` | `str` | contextual `Epoch` |
| `data.covariance_matrix[*].epoch` / `covarianceMatrix/EPOCH` | required `epochType` in each optional matrix | Covariance time tag under the segment time system | `OemCovarianceMatrix.epoch: Epoch` | `str` | contextual `Epoch` |

The OEM validator checks required structure, numeric state/covariance values, and the lexical
branch validity of these contextual epochs: calendar fields must describe a real date/time, and
numeric tags must be non-degenerate. It intentionally does not resolve `TIME_SYSTEM`, compare
calendar and numeric values, or infer MET/MRT/SCLK/ICD reference events; those remain context that
callers may supply for a future arithmetic or ordering API.

## OCM metadata reference-epoch audit

The OCM XSD declares `EPOCH_TZERO` as required `ndm:epochType`, and
`PREVIOUS_MESSAGE_EPOCH`, `NEXT_MESSAGE_EPOCH`, and `NEXT_LEAP_EPOCH` as optional
`ndm:epochType` elements. The shared schema type is intentionally broad, but the OCM book narrows
these four fields: section 6.2.2.3 permits relative time-tags in OCM data blocks, while table 6-3
(section 6.2.4) specifies `EPOCH_TZERO`, the message epochs, and `NEXT_LEAP_EPOCH` using the
absolute-time format in section 7.5.10. `NEXT_LEAP_EPOCH` is explicitly called an absolute time
tag. `TIME_SYSTEM` and the SCLK parameters change the interpretation of the reference epoch, not
the required calendar/ordinal spelling; for SCLK, the book specifically interprets `EPOCH_TZERO`
in UTC.

Accordingly, these four metadata fields use `CalendarEpoch` in Rust. KVN and XML parsing and
Serde deserialization reject numeric or invalid calendar spellings at construction, while Python
exposes a `str`/`str | None` API and validates through the same parser.
`START_TIME`, `STOP_TIME`, history-line epochs, and nested frame epochs remain `Epoch` because
their legal relative/absolute form still depends on OCM context or a separate conditional rule.

Metadata validation preserves the SCLK conditional boundary from ODM table 6-3: when
`TIME_SYSTEM=SCLK`, both `SCLK_OFFSET_AT_EPOCH` and `SCLK_SEC_PER_SI_SEC` must be present. KVN
parsing does not manufacture those optional values, so KVN and XML share the same omission
semantics. Their XSD time-unit restrictions and the conditional `NEXT_LEAP_TAIMUTC` requirement
are checked without resolving physical time.

The same absolute-format rule applies to the four nested reference-frame epochs
(`TRAJ_FRAME_EPOCH`, `OEB_PARENT_FRAME_EPOCH`, `COV_FRAME_EPOCH`, and `MAN_FRAME_EPOCH`). It also
applies to trajectory `USEABLE_START_TIME` and `USEABLE_STOP_TIME`: ODM §6.2.5 and table 6-4
explicitly refer these values to §7.5.10, even though the XSD uses the broad `epochType` union.
They use `CalendarEpoch` in Rust with a Python string surface. Their ordering and
relation to the data history still need the OCM time context; that ordering question is separate
from the already-established absolute lexical form.

## OCM scalar inventory

All entries are XML `ndm:epochType`. “Required” and “optional” below reproduce the OCM 3.0 XSD.
The header, the four audited metadata reference epochs, the four nested frame-reference epochs,
and the two trajectory usable-span fields use `CalendarEpoch`; the remaining Rust fields use
`Epoch`/`Option<Epoch>`. The Python binding exposes all of them as `str`/`str | None`.

| Block / XML name | Occurrence | Book meaning and governing context | Category / context |
| --- | --- | --- | --- |
| Header `CREATION_DATE` | required | Absolute UTC creation time | `CalendarEpoch` |
| Metadata `EPOCH_TZERO` | required | Absolute reference epoch for all relative data-block times; normally uses `TIME_SYSTEM`, with the book's special SCLK interpretation | `CalendarEpoch` plus `TimeContext` |
| Metadata `PREVIOUS_MESSAGE_EPOCH` | optional | Absolute creation epoch of the previous message; OCM absolute-time context | `CalendarEpoch` plus `TimeContext` |
| Metadata `NEXT_MESSAGE_EPOCH` | optional | Absolute anticipated/actual epoch of the next message; OCM absolute-time context | `CalendarEpoch` plus `TimeContext` |
| Metadata `START_TIME` | optional | Explicitly relative or absolute earliest-data time | contextual `Epoch` |
| Metadata `STOP_TIME` | optional | Explicitly relative or absolute latest-data time | contextual `Epoch` |
| Metadata `NEXT_LEAP_EPOCH` | optional | Explicitly absolute next leap-second epoch | `CalendarEpoch` plus `TimeContext` |
| Trajectory `TRAJ_FRAME_EPOCH` | optional | Absolute frame epoch when not intrinsic; defaults conceptually to `EPOCH_TZERO` | `CalendarEpoch` plus `TimeContext` |
| Trajectory `USEABLE_START_TIME` | optional | Absolute usable-span start formatted under 7.5.10 | `CalendarEpoch` plus `TimeContext` |
| Trajectory `USEABLE_STOP_TIME` | optional | Absolute usable-span stop formatted under 7.5.10 | `CalendarEpoch` plus `TimeContext` |
| Physical `OEB_PARENT_FRAME_EPOCH` | optional | Absolute OEB parent-frame epoch; default shown as `EPOCH_TZERO` | `CalendarEpoch` plus `TimeContext` |
| Covariance `COV_FRAME_EPOCH` | optional | Absolute covariance-frame epoch; default shown as `EPOCH_TZERO` | `CalendarEpoch` plus `TimeContext` |
| Maneuver `MAN_PREV_EPOCH` | optional | Completion time of the previous maneuver in the selected OCM time context | contextual `Epoch` |
| Maneuver `MAN_NEXT_EPOCH` | optional | Start time of the next maneuver in the selected OCM time context | contextual `Epoch` |
| Maneuver `MAN_FRAME_EPOCH` | optional | Absolute maneuver-frame epoch when not intrinsic | `CalendarEpoch` plus `TimeContext` |
| Maneuver `DC_WIN_OPEN` | conditionally required by the book | Duty-cycle window start; its ordering is contextual | contextual `Epoch` |
| Maneuver `DC_WIN_CLOSE` | conditionally required by the book | Duty-cycle window end; its ordering is contextual | contextual `Epoch` |
| Maneuver `DC_EXEC_START` | conditionally required by the book | Duty-cycle execution start, constrained against the window | contextual `Epoch` |
| Maneuver `DC_EXEC_STOP` | conditionally required by the book | Duty-cycle execution stop, constrained against the window | contextual `Epoch` |
| Maneuver `DC_REF_TIME` | conditionally required by the book | Explicitly either SI seconds relative to `EPOCH_TZERO` or an absolute epoch | contextual `Epoch` |
| Perturbations `SW_DATA_EPOCH` | optional | Epoch of the space-weather data under OCM time context | contextual `Epoch` pending a stronger book rule |
| Orbit determination `OD_EPOCH` | required within optional OD block | Explicitly relative or absolute solved-for-state time tag under `TIME_SYSTEM` | contextual `Epoch` |

The XSD makes the duty-cycle scalar fields optional because their requirement depends on
`DC_TYPE`; that conditional rule belongs in semantic validation rather than the scalar type.
The same separation applies to `SCLK_OFFSET_AT_EPOCH` and `SCLK_SEC_PER_SI_SEC`: the XSD marks
them optional, while ODM table 6-3 requires both when `TIME_SYSTEM=SCLK`. The Rust validator
enforces that conditional presence and the XSD seconds-only units without resolving clock values.

## OCM history-line inventory

| Location / XML name | XSD type | Book grammar | Current Rust/Python representation | Proposed category |
| --- | --- | --- | --- | --- |
| `traj[*].traj_lines[*].epoch` / `trajLine` first token | required repeated `xsd:string` | Time tag followed by the selected `TRAJ_TYPE` elements; relative values reference `EPOCH_TZERO` | Rust `Epoch`; Python `str` | contextual `Epoch` implemented |
| `cov[*].cov_lines[*].epoch` / `covLine` first token | required repeated `xsd:string` | Time tag followed by the selected covariance elements | Rust `Epoch`; Python `str` | contextual `Epoch` implemented |
| `man[*].man_lines[*].epoch` / `manLine` first token | required repeated `xsd:string` | First `MAN_COMPOSITION` element must be exactly one of `TIME_ABSOLUTE` or `TIME_RELATIVE`; absolute uses calendar/ordinal format and relative uses SI seconds from `EPOCH_TZERO` | Rust `Epoch`; Python `str` | contextual `Epoch` implemented with composition validation |

These three fields were the highest-risk finding from the inventory. All three parse their first
token into `Epoch`, and OCM validation rejects invalid calendar fields and degenerate numeric
spellings. Trajectory and covariance validation also enforces ODM §§6.2.2.4–6.2.2.5 and
§§6.2.5.6/6.2.7.6: every history record within one block must use the same calendar or numeric
branch, must not duplicate a time value, and must be strictly increasing. Maneuver validation
additionally requires the composition time tag to be first, selects the legal epoch branch
(`TIME_ABSOLUTE` or `TIME_RELATIVE`), and rejects duplicate values. Python exposes the
value as `str` while sharing the same mutation-time validation.

The 10k trajectory benchmark records 5.85–6.03 ms XML parsing, 4.02–4.03 ms KVN parsing, and about
2.21 MB of retained trajectory/value storage after parsing. The `CalendarEpoch` narrowing is
`repr(transparent)` over the existing inline value, so it does not change the trajectory record
layout.

## Context and conditional rules needed before conversion

An OCM `TimeContext` needs, at minimum:

- the original `TIME_SYSTEM`, preserving unknown ICD-defined values;
- `EPOCH_TZERO`;
- `SCLK_OFFSET_AT_EPOCH` and `SCLK_SEC_PER_SI_SEC` when SCLK is selected;
- external MET/MRT reference-event information when applicable;
- the message-provided leap-second/EOP values without silently replacing them with a library table.

The typed wire parser classifies spelling, calendar-field validity, and a compact same-branch
ordering key once without attempting physical-time conversion. Contextual adoption outside OPM still
covers scalar field categories and time-system-aware comparisons; no OCM parsed or generated message
accepts an unvalidated history epoch token. Ordering rules such as `START_TIME <= STOP_TIME`,
usable-span containment, and duty-cycle window ordering must wait until both operands can be
resolved under the same context. The context-free portion of the duty-cycle contract is now
implemented: `DC_TYPE` drives the required field set, `TIME_AND_ANGLE` requires its angle/trigger
fields, pulse durations use finite non-negative seconds, and period must not be shorter than
duration. Optional maneuver epoch fields must also follow the `MAN_COMPOSITION` branch, and the
explicit ODM schedule constraints `DC_EXEC_START >= DC_WIN_OPEN` and
`DC_EXEC_STOP <= DC_WIN_CLOSE` are enforced. Broader span/window comparisons remain contextual.

## Resulting migration order

1. Share one single-pass CCSDS lexical/calendar parser while retaining `Epoch` at existing fields.
2. Keep all OCM history-line epochs on the validated union; maneuver selects its legal branch from
   `MAN_COMPOSITION`.
3. Move unconditional fields, beginning with header `CREATION_DATE`, to `CalendarEpoch`.
4. Introduce borrowed segment/message `TimeContext` for comparison and optional physical-time
   conversion; do not duplicate it in history records.

This inventory confirms that a global replacement of `Epoch` with a physical-time library type
would be incorrect: the wire model must preserve relative decimals, ordinal dates, caller-defined
time systems, and arbitrary allowed fractional precision.
