# ACM/OCM epoch field inventory

This inventory records the remaining ACM and OCM epoch audit against the XML schemas and the
CCSDS books. The XSD defines the wire element and lexical type; the book supplies field semantics
and the stricter time-format rules that the shared `epochType` union cannot express.

## Classification

| Message | Field(s) | XSD declaration | Book evidence | Core representation |
| --- | --- | --- | --- | --- |
| ACM | `EPOCH_TZERO` | `ndmxml-4.0.0-acm-2.0.xsd`, `ndm:epochType` | ADM 504.0-B-2 §5.3.3 and §6.8.9 require the reference epoch's calendar/ordinal time format | `CalendarEpoch` |
| ACM | `NEXT_LEAP_EPOCH` | `ndmxml-4.0.0-acm-2.0.xsd`, `ndm:epochType` | ADM §5.3.3 describes it as an absolute time tag; §6.8.9 defines its calendar/ordinal format | `Option<CalendarEpoch>` |
| ACM | `START_TIME`, `STOP_TIME` | `ndmxml-4.0.0-acm-2.0.xsd`, `ndm:epochType` | ADM §5.3.3 permits either a relative tag from `EPOCH_TZERO` or an absolute tag | `Option<Epoch>` |
| ACM | `MAN_BEGIN_TIME`, `MAN_END_TIME` | `ndmxml-4.0.0-acm-2.0.xsd`, `ndm:relTimeType` (`xsd:double`) | ADM §5.3.8 requires relative seconds from `EPOCH_TZERO`; §6.8.4–§6.8.5 define finite fixed/scientific numeric forms | `Option<RelativeTime>` |
| OCM | `EPOCH_TZERO`, `PREVIOUS_MESSAGE_EPOCH`, `NEXT_MESSAGE_EPOCH`, `NEXT_LEAP_EPOCH` | `ndmxml-4.0.0-ocm-3.0.xsd`, `ndm:epochType` | ODM §6.2.3 and §7.5.10 identify these as absolute/calendar reference epochs | `CalendarEpoch` / `Option<CalendarEpoch>` |
| OCM | `START_TIME`, `STOP_TIME`, history-line epochs, maneuver predecessor/successor and duty-cycle tags | `ndmxml-4.0.0-ocm-3.0.xsd`, `ndm:epochType` | ODM §6.2.2.3 permits relative or absolute time tags; §6.2.2.5 requires one branch consistently within a data block | `Epoch` |
| OCM trajectory | `USEABLE_START_TIME`, `USEABLE_STOP_TIME` | `ndmxml-4.0.0-ocm-3.0.xsd`, `ndm:epochType` | ODM §6.2.5 and table 6-4 refer these absolute usable-span values to §7.5.10 | `Option<CalendarEpoch>` |
| OCM | `SW_DATA_EPOCH`, `OD_EPOCH` | `ndmxml-4.0.0-ocm-3.0.xsd`, `ndm:epochType` | ODM tables 6-10 and 6-11 give field-specific epochs, while the general OCM time-tag rule remains applicable | `Option<Epoch>` / `Epoch` |
| OCM | frame-reference epochs (`TRAJ_FRAME_EPOCH`, `OEB_PARENT_FRAME_EPOCH`, `COV_FRAME_EPOCH`, `MAN_FRAME_EPOCH`) | `ndmxml-4.0.0-ocm-3.0.xsd`, `ndm:epochType` | ODM §§6.2.5–6.2.8 point to §7.5.10 absolute formatting | `CalendarEpoch` / `Option<CalendarEpoch>` |

The ACM metadata change is a lexical narrowing at the typed boundary: the XSD still exposes the
broad union, but the book requires a calendar/ordinal reference epoch and absolute leap epoch.
ACM `START_TIME`/`STOP_TIME` and the OCM rows marked `Epoch` retain both XSD branches because their
meaning is selected by `TIME_SYSTEM`, `EPOCH_TZERO`, and the containing data-block rules. No time
arithmetic or branch comparison is introduced.

`AcmAttitudeDetermination::ad_epoch` is currently a legacy KVN-only field in the Rust/Python model;
it is absent from both the ACM 2.0 XSD and the ADM 504.0-B-2 keyword table. It is therefore not
treated as conformance evidence and should be resolved separately before being advertised or
removed in a breaking release.

## Executable evidence

- Rust KVN/XML rejection of numeric ACM metadata reference epochs:
  `ccsds-ndm/tests/acm_calendar_epoch.rs`.
- Python setter rejection for the same fields: `bindings/python/tests/test_acm.py`.

`RelativeTime` is deliberately separate from the numeric branch of `Epoch`: `relTimeType` is
`xsd:double`, not the common `epochType` union. It preserves finite fixed/scientific spelling in
the inline model, rejects unsupported NaN/Inf/negative-zero values required by ADM §6.8.5, and
performs no conversion.
