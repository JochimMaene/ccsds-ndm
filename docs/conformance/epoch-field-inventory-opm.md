# OPM epoch field inventory

Status: implemented slice, not a blanket OPM conformance claim. Scope: OPM 3.0.

## Evidence

- The OPM schema `data/xsd/ndmxml-4.0.0-opm-3.0.xsd` declares `REF_FRAME_EPOCH` and
  `MAN_EPOCH_IGNITION` as `ndm:epochType`; the common schema declares the required state-vector
  `EPOCH` with the same broad XSD union.
- ODM book section 3.2.3 lists `REF_FRAME_EPOCH` and section 3.2.4 lists the state-vector
  `EPOCH` and `MAN_EPOCH_IGNITION`; each explicitly refers to section 7.5.10 formatting.
- ODM section 7.5.10 permits calendar and ordinal time tags. The MET/MRT note in section 3.2.3.2
  still requires the ordinal day form, not the XSD numeric branch; `TIME_SYSTEM` remains context
  for interpreting the tag.
- RDM uses the shared Rust `StateVector` type. RDM section 3.5 and syntax rule 5.3.3.5 likewise
  require calendar or ordinal tags, so changing that shared field does not broaden RDM's syntax.

## Decision

OPM reference-frame epochs, state-vector epochs, and maneuver ignition epochs now use
`CalendarEpoch` in Rust. KVN and XML parsing reject numeric or invalid calendar spellings, while
Python retains its string constructor/getter/setter surfaces and raises `ValueError` for the same
invalid values. No physical-time conversion is introduced.
