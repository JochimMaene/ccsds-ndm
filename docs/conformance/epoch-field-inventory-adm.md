# ADM header epoch audit

Status: implemented slice, not a blanket ADM conformance claim. Scope: the shared
`admHeader` used by APM 2.0, AEM 2.0, and ACM 2.0.

## Evidence

- The XML schema `data/xsd/ndmxml-4.0.0-common-4.0.xsd` declares `admHeader/CREATION_DATE` as
  required `ndm:epochType`, so the wire model must continue to accept the XSD calendar/ordinal
  and numeric union at the schema boundary.
- ADM book sections 3.2.2.2 (APM), 4.2.2.2 (AEM), and 5.3.2.2 (ACM) require `CREATION_DATE` to
  be the UTC time when the file was created. Their tables describe it as file creation date/time
  in UTC and refer to section 6.8.9 for calendar/ordinal time-tag formatting.
- Section 6.8.9 permits calendar and ordinal spellings, optional fractional seconds, and an
  optional `Z` terminator. It does not define numeric relative creation dates.

## Decision

`AdmHeader.creation_date` now uses `CalendarEpoch`, matching the earlier ODM header slice. KVN and
XML parsing/deserialization reject numeric, empty, and invalid calendar values before a message is
constructed. The Python `AdmHeader` constructor/getter/setter remains a string API and raises
`ValueError` for the same invalid values.

The generic `NdmHeader` remains on `Epoch`. It has no active APM/AEM/ACM message owner in the Rust
model, so migrating it without a message-specific book audit would broaden the change beyond this
slice.
