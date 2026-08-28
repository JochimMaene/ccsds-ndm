# CDM/TDM/RDM epoch field inventory

This inventory records the next epoch-boundary slice reviewed against the XML schemas and the
CCSDS books. The XSD remains authoritative for element names and wire structure; the books supply
the lexical representation rules that the shared `epochType` union cannot encode on its own.

## Classification

| Message | Field(s) | XSD declaration | Book evidence | Core representation |
| --- | --- | --- | --- | --- |
| CDM | `CREATION_DATE`, `TCA`, `START_SCREEN_PERIOD`, `STOP_SCREEN_PERIOD`, `SCREEN_ENTRY_TIME`, `SCREEN_EXIT_TIME` | `ccsds-ndm/data/xsd/ndmxml-4.0.0-cdm-1.0.xsd`, `ndm:epochType` | CDM 508.0-B-1 §§3.2–3.3 and §6.3.2.6; §6.2.3.4 says all CDM time tags are UTC | `CalendarEpoch` |
| CDM/RDM common OD | `TIME_LASTOB_START`, `TIME_LASTOB_END` | `ccsds-ndm/data/xsd/ndmxml-4.0.0-common-4.0.xsd`, `ndm:epochType` | CDM §3.5.2 and RDM §3.5; both point to calendar/ordinal formatting | `Option<CalendarEpoch>` |
| TDM | `CREATION_DATE`, `START_TIME`, `STOP_TIME` | `ccsds-ndm/data/xsd/ndmxml-4.0.0-tdm-2.0.xsd`, `ndm:epochType` | TDM 503.0-B-2 §§3.2–3.3 and §4.3.9 | `CalendarEpoch` / `Option<CalendarEpoch>` |
| TDM | each data-record `EPOCH` timetag | `ccsds-ndm/data/xsd/ndmxml-4.0.0-tdm-2.0.xsd`, `trackingDataObservationType/EPOCH`, `ndm:epochType` | TDM §§3.4.1, 3.4.8 and §4.3.9: the tag is interpreted using `TIME_SYSTEM`, but the value still uses one of the two calendar/ordinal timetag formats | `CalendarEpoch` |
| RDM | `CREATION_DATE`; `EPOCH_TZERO`; optional `REF_FRAME_EPOCH`, `PREVIOUS_MESSAGE_EPOCH`, `NEXT_MESSAGE_EPOCH` | `ccsds-ndm/data/xsd/ndmxml-4.0.0-rdm-1.0.xsd`, `ndm:epochType` | RDM 508.1-B-1 §§3.2–3.4 and §5.3.3.5; previous/next epochs are explicitly UTC | `CalendarEpoch` / `Option<CalendarEpoch>` |
| RDM | `NOMINAL_REENTRY_EPOCH`, `REENTRY_WINDOW_START`, `REENTRY_WINDOW_END`; `NOMINAL_IMPACT_EPOCH`, `IMPACT_WINDOW_START`, `IMPACT_WINDOW_END` | `ccsds-ndm/data/xsd/ndmxml-4.0.0-rdm-1.0.xsd`, common re-entry/impact types, `ndm:epochType` | RDM §3.5 and §5.3.3.5 | `Option<CalendarEpoch>` |
| RDM | state-vector `EPOCH` | common `stateVectorType`, `ndm:epochType` | RDM §3.5 and §5.3.3.5 | `CalendarEpoch` (already shared with OPM) |

The XSD union also accepts signed numeric spellings for every row above. The corresponding book
sections require the two calendar-date or ordinal-date formats, so the typed boundary now rejects
numeric values while preserving the original spelling. TDM `EPOCH` remains semantically dependent
on the segment's `TIME_SYSTEM`; `CalendarEpoch` expresses only the mandated lexical branch and
does not perform time-system conversion.

RDM's documented `N/A` value for `NEXT_MESSAGE_EPOCH` is represented as `None`; KVN null handling
accepts both `N/A` and `n/a`.

## Executable evidence

- Rust KVN/XML rejection and RDM `N/A` handling:
  `ccsds-ndm/tests/cdm_tdm_rdm_calendar_epoch.rs`.
- Python constructor rejection for representative fields:
  `bindings/python/tests/test_cdm.py`, `test_tdm.py`, and `test_rdm.py`.

No epoch arithmetic, timezone normalization, or cross-field ordering checks are introduced here.
