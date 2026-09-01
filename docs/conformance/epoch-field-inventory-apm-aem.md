# APM/AEM epoch field inventory

This inventory records the epoch fields reviewed for ADM 2.0 before the calendar-epoch migration.
The XML schema remains authoritative for element names and wire structure; the ADM Blue Book is
used for the field semantics and representation rule that the shared `epochType` union cannot
express by itself.

## Evidence

| Message | Field(s) | XSD declaration | Book requirement | Core representation |
| --- | --- | --- | --- | --- |
| APM | `data/EPOCH` | `ndmxml-4.0.0-apm-2.0.xsd`: `apmData/EPOCH`, `ndm:epochType` | ADM 504.0-B-2 §3.2.4 and §3.2.5.1; formatting is §6.8.9 | `CalendarEpoch` |
| APM | `maneuverParameters/MAN_EPOCH_START` | `ndmxml-4.0.0-common-4.0.xsd`: `attManeuverStateType/MAN_EPOCH_START`, `ndm:epochType` | ADM §3.2.4 and §6.8.9 | `CalendarEpoch` |
| AEM | metadata `START_TIME`, `USEABLE_START_TIME`, `USEABLE_STOP_TIME`, `STOP_TIME` | `ndmxml-4.0.0-aem-2.0.xsd`: `aemMetadata`, `ndm:epochType` | ADM §4.2.3 and §6.8.9; useable-span ordering is additionally described in §4.2.3 | `CalendarEpoch` |
| AEM | each attitude-state `EPOCH` | `ndmxml-4.0.0-common-4.0.xsd`: all nine AEM attitude-state types, `ndm:epochType` | ADM §4.2.4.7.2–§4.2.4.8.2 and §6.8.9 | `CalendarEpoch` |

The XSD `epochType` is a union that also admits signed numeric spellings. ADM §6.8.9, however,
requires every ADM time tag or epoch to use calendar-date or ordinal-date CCSDS time format. The
typed boundary therefore rejects numeric values for these fields while preserving the original
calendar/ordinal spelling (including an optional time-zone suffix accepted by the shared XSD
branch). This is a narrower rule than changing the global `Epoch` type, which remains necessary for
other NDM fields whose XSD union branch is contextual or relative.

`AdmHeader::creation_date` was already migrated to `CalendarEpoch`; it is UTC by the APM/AEM header
rules in §3.2.2.2 and §4.2.2.2.

## Executable evidence

- KVN and XML rejection of numeric APM epochs and maneuver starts:
  `ccsds-ndm/tests/adm_calendar_epoch.rs`.
- KVN and XML rejection of numeric AEM metadata and attitude-state epochs:
  `ccsds-ndm/tests/adm_calendar_epoch.rs`.
- Python constructors/setters enforce the same boundary in `bindings/python/tests/test_apm.py`
  and `bindings/python/tests/test_aem.py`.

No epoch arithmetic or cross-field time ordering is introduced by this change. Those checks require
a separate, explicit time-context design and remain outside this inventory slice.
