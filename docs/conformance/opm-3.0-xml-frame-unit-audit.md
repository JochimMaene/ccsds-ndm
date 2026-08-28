# OPM 3.0 XML Frame and Unit Audit

Scope: Rust generation of standalone OPM 3.0 XML. The NDM/XML 4.0.0 XSD set is the
authority for XML structure and lexical constraints. CCSDS 502.0-B-3 is used only for
semantics the schema does not express.

## Reference frames

| OPM field | XSD constraint | ODM semantics | Generation decision |
| --- | --- | --- | --- |
| `REF_FRAME` | Required, unrestricted `xsd:string` | Section 3.2.3.3 says values *should* come from its named set; another value may be defined in an ICD. | Keep as `String`. Do not add an enum or reject non-standard names. |
| `COV_REF_FRAME` | Optional, unrestricted `xsd:string` | Sections 3.2.4 and 3.2.4.11 describe RSW, RTN, and TNW; omission means the metadata `REF_FRAME` applies. | Keep as `Option<String>`. Do not add an enum. Interpretation and ICD context belong to the caller. |
| `MAN_REF_FRAME` | Required when a maneuver block is present, unrestricted `xsd:string` | Sections 3.2.4 and 3.2.4.11 describe RSW, RTN, and TNW. | Keep as `String`. Do not add an enum. |

The relevant declarations are in
[`ndmxml-4.0.0-opm-3.0.xsd`](../../ccsds-ndm/data/xsd/ndmxml-4.0.0-opm-3.0.xsd) and
[`ndmxml-4.0.0-common-4.0.xsd`](../../ccsds-ndm/data/xsd/ndmxml-4.0.0-common-4.0.xsd).
Because all three values are unrestricted strings in the schema, XSD-first generation must not
turn the book's frame lists into closed wire-format vocabularies. Existing rejection of blank
required frame values is semantic model validation, not an XSD enumeration rule.

## Units

Every OPM XML unit attribute is optional in the XSD. When present, it is restricted as follows:

| OPM values | Allowed XML `units` value |
| --- | --- |
| State-vector position and `SEMI_MAJOR_AXIS` | `km` |
| State-vector velocity and `MAN_DV_1`–`MAN_DV_3` | `km/s` |
| Keplerian angles | `deg` |
| `GM` | `km**3/s**2` or `KM**3/S**2` |
| `MASS` and `MAN_DELTA_MASS` | `kg` |
| `SOLAR_RAD_AREA` and `DRAG_AREA` | `m**2` |
| Position covariance terms | `km**2` |
| Position/velocity covariance terms | `km**2/s` |
| Velocity covariance terms | `km**2/s**2` |
| `MAN_DURATION` | `s` |

The Rust model represents all but the last row with closed unit enums whose serialized values
match the XSD enumerations. Invalid unit text therefore cannot be created through those safe
public types, and no additional pre-generation string validation is needed. Dimensionless OPM
values have no unit attribute in the model.

The ODM table 3-3 specifies the same units. Section 7.7.1 additionally governs optional unit text
in KVN; those presentation rules do not narrow the XML attributes beyond the XSD.

## Resolved defect

`MAN_DURATION` uses the shared [`TimeUnits`](../../ccsds-ndm/src/types.rs) enum. That enum exposes
both `Seconds` (`s`) and `Day` (`d`), while XSD `durationType` uses `timeUnits`, whose only allowed
value is `s`. A caller could consequently construct an otherwise valid OPM with
`man_duration.units = Some(TimeUnits::Day)` and generate XML containing
`<MAN_DURATION units="d">...`, which fails the official schema.

OPM maneuver validation now rejects `TimeUnits::Day` before generation, while preserving the
shared type for other message families. Regression tests cover rejection after public mutation
and successful generation with `TimeUnits::Seconds`. Auditing other message families that use
`Duration` remains separate scope. No other OPM frame or unit validation change is justified by
this audit.
