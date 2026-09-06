# OMM epoch field inventory

Status: implemented slice, not a blanket OMM conformance claim. Scope: OMM 3.0.

## Evidence

- The XML schema `ccsds-ndm/data/xsd/ndmxml-4.0.0-omm-3.0.xsd` declares `REF_FRAME_EPOCH` as an optional
  `ndm:epochType`, so the wire union remains broader than the field's semantic rule.
- ODM book section 4.2.3 describes `REF_FRAME_EPOCH` as the epoch of the reference frame and
  points to section 7.5.10 for its formatting. The examples are calendar and ordinal time tags.
- ODM section 7.5.11 supplies `TIME_SYSTEM` for non-creation-date epochs; this field's time system
  context is therefore retained separately from its required calendar/ordinal spelling.

## Decision

`OmmMetadata.ref_frame_epoch` and `MeanElements.epoch` use `Option<CalendarEpoch>` and
`CalendarEpoch`, respectively, in Rust. KVN and XML parsing reject numeric or invalid calendar
spellings, while Python keeps the existing string constructor, getter, and setter surfaces and
raises `ValueError` for the same invalid values.

The selected `TIME_SYSTEM` remains message context for interpreting these calendar/ordinal tags;
this slice does not introduce physical-time conversion or broader OMM time-context rules.
